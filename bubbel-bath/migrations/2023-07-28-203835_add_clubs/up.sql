CREATE TABLE club_profiles (
    id SERIAL PRIMARY KEY,
    owner INTEGER NOT NULL,
    name VARCHAR NOT NULL,
    dc_id INTEGER NOT NULL,
    description VARCHAR,
    display_name VARCHAR,
    pfp VARCHAR,
    banner VARCHAR
);
