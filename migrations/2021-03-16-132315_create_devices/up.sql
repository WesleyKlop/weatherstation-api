create table devices
(
    id         UUID      not null default uuid_generate_v4() primary key,

    location   varchar   not null,
    token      char(32)  not null,

    created_at timestamp not null default now(),
    updated_at timestamp null
);
