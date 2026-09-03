alter table agent_runs add column configuration_version integer not null default 1;
alter table agent_entries add column payload_version integer not null default 1;
alter table agent_records add column payload_version integer not null default 1;
