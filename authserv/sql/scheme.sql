drop table if exists "session";
drop table if exists "user_table";

create table "user_table"(
    id uuid primary key not null ,
    username varchar (40) not null unique ,
    password varchar (60) not null ,
    deleted_at timestamp
);

create table "session" (
    id uuid not null ,
    user_id uuid not null ,
    deleted_at timestamp  ,

    primary key (id, user_id) ,
    foreign key (user_id)
        references "user_table"(id)
)