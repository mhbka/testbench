create table if not exists users
(
    user_id   serial primary key,
    username  text unique not null,
    pw_hash   text not null
)


