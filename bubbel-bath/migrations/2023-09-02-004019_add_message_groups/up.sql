CREATE TABLE message_groups (
    message_group_id INT PRIMARY KEY,
    name VARCHAR,
    dc_id INT NOT NULL
);

CREATE TABLE message_group_members (
    user_id INT PRIMARY KEY,
    message_group_id INT NOT NULL,
    FOREIGN KEY (message_group_id) REFERENCES message_groups (message_group_id)
);
