CREATE TABLE dogs(
    id       SERIAL PRIMARY KEY,
    name     TEXT NOT NULL
);

INSERT INTO dogs(name) VALUES ('Fido'), ('Argos'), ('Balto');
