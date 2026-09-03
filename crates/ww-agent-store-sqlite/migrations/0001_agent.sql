create table if not exists agent_runs (
    id                  text primary key,
    configuration_json  text not null,
    created_at          text not null,
    version             integer not null
);

create table if not exists agent_entries (
    id            text primary key,
    run_id        text not null references agent_runs(id),
    ordinal       integer not null,
    created_at    text not null,
    kind          text not null,
    payload_json  text not null,
    unique(run_id, ordinal)
);

create index if not exists agent_entries_run_idx
    on agent_entries(run_id, ordinal);

create table if not exists agent_records (
    run_id        text not null references agent_runs(id),
    sequence      integer not null,
    recorded_at   text not null,
    kind          text not null,
    payload_json  text not null,
    primary key(run_id, sequence)
);
