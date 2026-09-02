pragma foreign_keys = on;
pragma journal_mode = wal;

create table if not exists executions (
    id                    text primary key,
    kind                  text not null,
    status                text not null,
    configuration_digest  text not null,
    cancel_requested      integer not null default 0,
    cancel_reason_json    text null,
    result_artifact_json  text null,
    error_json            text null,
    created_at            text not null,
    started_at            text null,
    finished_at           text null,
    deadline              text null,
    version               integer not null
);

create index if not exists executions_status_idx
    on executions(status, created_at);

create table if not exists execution_events (
    id                text primary key,
    execution_id      text not null references executions(id),
    sequence          integer not null,
    occurred_at       text not null,
    kind              text not null,
    payload_version   integer not null,
    visibility        text not null,
    payload_json      text not null,
    unique(execution_id, sequence)
);

create index if not exists execution_events_execution_idx
    on execution_events(execution_id, sequence);

create table if not exists artifacts (
    id            text primary key,
    digest        text not null unique,
    media_type    text not null,
    size_bytes    integer not null,
    storage_uri   text not null,
    created_at    text not null
);

pragma user_version = 1;
