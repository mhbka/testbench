create table users
(
    user_id         serial primary key,
    username        text unique not null,
    password_hash   text not null
)


