CREATE TABLE friend_connections (
    id SERIAL PRIMARY KEY,
    send_user_id INTEGER NOT NULL,
    recv_user_id INTEGER NOT NULL
);
