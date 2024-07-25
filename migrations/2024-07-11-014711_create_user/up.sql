-- Your SQL goes here
CREATE TABLE IF NOT EXISTS user (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(255) NULL,
    username VARCHAR(255) NULL,
    password VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    rg VARCHAR(255) NULL,
    cpf VARCHAR(255) NULL,
    phone VARCHAR(255) NULL,
    number INTEGER NULL,
    complement VARCHAR(255) NULL,
    role_id INTEGER NULL,
    address_id INTEGER NULL,
    FOREIGN KEY (role_id) REFERENCES role(id),
    FOREIGN KEY (address_id) REFERENCES address(id)
);