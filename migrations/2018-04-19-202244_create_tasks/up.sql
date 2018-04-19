create table tasks (
    id integer not null primary key,
    description text not null,
    created_at timestamp not null,
    updated_at timestamp
);
