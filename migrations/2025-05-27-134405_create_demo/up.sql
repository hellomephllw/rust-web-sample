create table rust_user (
    id int not null auto_increment primary key,
    name varchar(20) not null
) default charset = utf8mb4 comment = '用户';