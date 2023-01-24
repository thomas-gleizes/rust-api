-- Your SQL goes here
create table posts
(
    id     int auto_increment primary key,
    title  varchar(255) not null,
    body   varchar(255) not null,
    posted boolean      not null default false
)
