use chrono::Utc;
use serde_json::json;
use std::{env, path::PathBuf};
use ww_agent_core::{
    AgentAppend, AgentAssistantContent, AgentEntry, AgentEntryData, AgentEntryId, AgentRecord,
    AgentRecordData, AgentRunId, AgentStore, AgentTerminalResult, DurableAssistantMessage,
    ModelAttemptId, NewAgentRun, reduce_agent_history,
};
use ww_agent_provider::CompletionReason;
use ww_agent_store_sqlite::SqliteAgentStore;

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let command = args.next().ok_or("missing command")?;
    let db = PathBuf::from(args.next().ok_or("missing database path")?);
    let store = SqliteAgentStore::new(db);
    store.migrate().await?;

    match command.as_str() {
        "seed" => {
            let run_id = AgentRunId::new();
            let initial = AgentEntry {
                id: AgentEntryId::new(),
                run_id,
                ordinal: 1,
                created_at: Utc::now(),
                data: AgentEntryData::UserInput {
                    text: "process-boundary".to_owned(),
                },
            };
            store
                .create_run(NewAgentRun {
                    id: run_id,
                    configuration: json!({"fixture": true}),
                    created_at: Utc::now(),
                    initial_entry: initial,
                })
                .await?;

            let attempt_id = ModelAttemptId::new();
            let assistant_id = AgentEntryId::new();
            let assistant = AgentEntry {
                id: assistant_id,
                run_id,
                ordinal: 2,
                created_at: Utc::now(),
                data: AgentEntryData::AssistantMessage {
                    attempt_id,
                    message: DurableAssistantMessage {
                        content: vec![AgentAssistantContent::Text {
                            text: "done".to_owned(),
                        }],
                        stop_reason: CompletionReason::Stop,
                        usage: None,
                        provider_request_id: Some("fixture-request".to_owned()),
                    },
                },
            };
            let now = Utc::now();
            store
                .append(AgentAppend {
                    run_id,
                    expected_version: 1,
                    entries: vec![assistant],
                    records: vec![
                        AgentRecord {
                            run_id,
                            sequence: 1,
                            recorded_at: now,
                            data: AgentRecordData::ModelAttemptStarted {
                                attempt_id,
                                request_ordinal: 1,
                            },
                        },
                        AgentRecord {
                            run_id,
                            sequence: 2,
                            recorded_at: now,
                            data: AgentRecordData::ModelAttemptCompleted {
                                attempt_id,
                                assistant_entry_id: assistant_id,
                            },
                        },
                        AgentRecord {
                            run_id,
                            sequence: 3,
                            recorded_at: now,
                            data: AgentRecordData::AgentResultCommitted {
                                result: AgentTerminalResult::Succeeded {
                                    assistant_entry_id: assistant_id,
                                },
                            },
                        },
                    ],
                })
                .await?;
            println!("{run_id}");
        }
        "inspect" => {
            let id = args.next().ok_or("missing run id")?;
            let id = AgentRunId::from_uuid(uuid::Uuid::parse_str(&id)?);
            let history = store.load_history(id).await?;
            let state = reduce_agent_history(id, &history.entries, &history.records)?;
            println!("{}", serde_json::to_string(&state)?);
        }
        other => return Err(format!("unknown command: {other}").into()),
    }
    Ok(())
}
