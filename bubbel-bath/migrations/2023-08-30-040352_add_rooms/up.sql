CREATE TABLE message_rooms (
    room_id INT PRIMARY KEY,
    name VARCHAR,
    club_id INT NOT NULL,
    dc_id INT NOT NULL,
    FOREIGN KEY (club_id) REFERENCES club_profiles (id)
);

CREATE TABLE message_room_members (
    user_id INT PRIMARY KEY,
    room_id INT NOT NULL,
    FOREIGN KEY (room_id) REFERENCES message_rooms (room_id)
);
