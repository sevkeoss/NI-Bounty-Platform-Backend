CREATE TABLE IF NOT EXISTS bounties (
    id              SERIAL PRIMARY KEY,
    description     VARCHAR(255) NOT NULL,
    price           DOUBLE PRECISION NOT NULL
);