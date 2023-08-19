CREATE TABLE data_channels (
    id SERIAL PRIMARY KEY,
    data_channel VARCHAR NOT NULL
);

CREATE TABLE data_chunks (
    id SERIAL PRIMARY KEY,
    data_chunk VARCHAR NOT NULL
);
