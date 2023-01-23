-- Your SQL goes here
CREATE TABLE posts
(
    id        int auto_increment primary key,
    title     varchar(255) not null default '',
    content   text         not null default '',
    posted_at timestamp    not null default current_timestamp
)
