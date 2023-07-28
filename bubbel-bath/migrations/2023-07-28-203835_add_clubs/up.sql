CREATE TABLE club_profiles (
    id SERIAL PRIMARY KEY,
    owner SERIAL NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR,
    display_name VARCHAR,
    pfp VARCHAR,
    banner VARCHAR
);
