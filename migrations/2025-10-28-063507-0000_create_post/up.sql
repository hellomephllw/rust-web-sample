-- Your SQL goes here
create table rust_post (
    id int not null auto_increment primary key,
    title varchar(255) not null,
    content longtext not null,
    published tinyint not null default 0,
    created_at timestamp default current_timestamp not null,
    updated_at timestamp default current_timestamp not null on update current_timestamp
) default charset = utf8mb4 comment = '帖子';