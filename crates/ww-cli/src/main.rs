use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use serde_json::json;
use std::{path::PathBuf, str::FromStr};
use ww_sdk::WorkWeaveSdk;
use ww_types::{CancelReason, ExecutionId, ExecutionKind};

#[derive(Parser)]
#[command(name = "ww", version, about = "WorkWeave Engine CLI")]
struct Cli {
    #[arg(long, env = "WW_DB", default_value = ".workweave/runtime.db")]
    db: PathBuf,
    #[arg(long, env = "WW_ARTIFACTS", default_value = ".workweave/artifacts")]
    artifacts: PathBuf,
    #[arg(long, global = true)]
    json: bool,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Run(RunArgs),
    Artifact(ArtifactArgs),
}

#[derive(Args)]
struct RunArgs {
    #[command(subcommand)]
    command: RunCommand,
}

#[derive(Subcommand)]
enum RunCommand {
    Create {
        #[arg(long, default_value = "synthetic")]
        kind: String,
        #[arg(long, default_value = "{}")]
        configuration: String,
    },
    Start {
        id: String,
    },
    Cancel {
        id: String,
        #[arg(long, default_value = "operator")]
        code: String,
        #[arg(long)]
        message: Option<String>,
    },
    Succeed {
        id: String,
    },
    Fail {
        id: String,
        #[arg(long)]
        error: String,
    },
    SettleCancelled {
        id: String,
    },
    Inspect {
        id: String,
    },
    Events {
        id: String,
        #[arg(long, default_value_t = 0)]
        after: u64,
        #[arg(long, default_value_t = 100)]
        limit: usize,
    },
}

#[derive(Args)]
struct ArtifactArgs {
    #[command(subcommand)]
    command: ArtifactCommand,
}

#[derive(Subcommand)]
enum ArtifactCommand {
    Put {
        path: PathBuf,
        #[arg(long, default_value = "application/octet-stream")]
        media_type: String,
    },
}

fn parse_id(value: &str) -> Result<ExecutionId> {
    ExecutionId::from_str(value).with_context(|| format!("invalid execution id: {value}"))
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    if let Some(parent) = cli.db.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    let sdk = WorkWeaveSdk::open_local(&cli.db, &cli.artifacts).await?;

    match cli.command {
        Command::Run(args) => match args.command {
            RunCommand::Create {
                kind,
                configuration,
            } => {
                let kind = ExecutionKind::new(kind).map_err(anyhow::Error::msg)?;
                let record = sdk
                    .create_execution(kind, configuration.as_bytes(), None)
                    .await?;
                print_value(cli.json, &record)?;
            }
            RunCommand::Start { id } => {
                print_value(cli.json, &sdk.start_execution(parse_id(&id)?).await?)?
            }
            RunCommand::Cancel { id, code, message } => {
                print_value(
                    cli.json,
                    &sdk.request_cancel(parse_id(&id)?, CancelReason::new(code, message))
                        .await?,
                )?;
            }
            RunCommand::Succeed { id } => print_value(
                cli.json,
                &sdk.succeed_execution(parse_id(&id)?, None).await?,
            )?,
            RunCommand::Fail { id, error } => {
                let error =
                    serde_json::from_str(&error).unwrap_or_else(|_| json!({"message": error}));
                print_value(cli.json, &sdk.fail_execution(parse_id(&id)?, error).await?)?;
            }
            RunCommand::SettleCancelled { id } => {
                print_value(cli.json, &sdk.settle_cancelled(parse_id(&id)?, None).await?)?;
            }
            RunCommand::Inspect { id } => {
                let inspection = sdk.inspect_execution(parse_id(&id)?).await?;
                if cli.json {
                    println!("{}", serde_json::to_string_pretty(&inspection.record)?);
                } else {
                    println!("id: {}", inspection.record.id);
                    println!("kind: {}", inspection.record.kind);
                    println!("status: {}", inspection.record.status);
                    println!("version: {}", inspection.record.version);
                    println!("cancel_requested: {}", inspection.record.cancel_requested);
                }
            }
            RunCommand::Events { id, after, limit } => {
                for event in sdk.execution_events(parse_id(&id)?, after, limit).await? {
                    println!("{}", serde_json::to_string(&event)?);
                }
            }
        },
        Command::Artifact(args) => match args.command {
            ArtifactCommand::Put { path, media_type } => {
                let bytes = tokio::fs::read(&path)
                    .await
                    .with_context(|| format!("read {}", path.display()))?;
                let artifact = sdk.put_artifact(&bytes, media_type).await?;
                print_value(cli.json, &artifact)?;
            }
        },
    }
    Ok(())
}

fn print_value<T: serde::Serialize + std::fmt::Debug>(json_output: bool, value: &T) -> Result<()> {
    if json_output {
        println!("{}", serde_json::to_string_pretty(value)?);
    } else {
        println!("{value:?}");
    }
    Ok(())
}
