create table if not exists agent_execution_links (
    agent_run_id  text primary key references agent_runs(id),
    execution_id  text not null unique references executions(id)
);
