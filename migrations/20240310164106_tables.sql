-- Add migration script here
create table users (
    id uuid primary key default gen_random_uuid(),
    username varchar(255) unique not null,
    created_at timestamptz not null default now()
);
