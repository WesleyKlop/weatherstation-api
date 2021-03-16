create extension if not exists "uuid-ossp";

create table measurements
(
    id             UUID      not null default uuid_generate_v4() primary key,

    humidity       float     not null,
    temperature    float     not null,
    carbon_dioxide float     not null,

    created_at     timestamp not null default now(),
    updated_at     timestamp null
);