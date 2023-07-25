
# DB
# docker run --rm -d -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust --name postgres-rustwi postgres:15-alpine
# docker exec -it postgres-rustwi psql -U postgres

CREATE TABLE tweets
(
    id        serial primary key,
    message   text        not null,
    posted_at timestamptz not null
);

INSERT INTO tweets VALUES (1, '始めてのツイート', '2023-01-02 03:04:05');
INSERT INTO tweets VALUES (2, '久しぶりのツイート', '2023-06-07 08:09:10');