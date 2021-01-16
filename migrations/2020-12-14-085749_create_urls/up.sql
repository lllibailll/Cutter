create table if not exists urls
(
    id          int auto_increment
        primary key,
    shorten_url varchar(100) not null,
    target      varchar(500) not null,
    constraint urls_shorten_url_uindex
        unique (shorten_url)
);
