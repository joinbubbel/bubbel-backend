CREATE TABLE user_profiles (
    user_id SERIAL PRIMARY KEY,
    name VARCHAR,
    description VARCHAR,
    display_name VARCHAR,
    pfp VARCHAR,
    banner VARCHAR
);
