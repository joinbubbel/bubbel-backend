CREATE TABLE message_groups (
    message_group_id SERIAL PRIMARY KEY,
    name VARCHAR,
    dc_id INT NOT NULL
);

CREATE TABLE message_group_members (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    message_group_id INT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (message_group_id) REFERENCES message_groups (message_group_id)
);
