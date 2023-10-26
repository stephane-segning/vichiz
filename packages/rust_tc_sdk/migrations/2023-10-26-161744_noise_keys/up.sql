-- Your SQL goes here
CREATE TABLE noise_keys
(
    id      VARCHAR PRIMARY KEY NOT NULL,
    private BINARY NOT NULL,
    public  BINARY NOT NULL
)