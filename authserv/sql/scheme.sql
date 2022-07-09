drop table if exists "SESSION";
drop table if exists "USER";

create table "USER"(
    id uuid primary key not null ,
    username varchar (40) not null unique ,
    password varchar (60) not null ,
    deleted_at timestamp
);

create table "SESSION" (
    id uuid not null ,
    user_id uuid not null ,
    expires timestamp not null ,
    deleted_at timestamp  ,

    primary key (id, user_id) ,
    foreign key (user_id)
        references "USER"(id)
)