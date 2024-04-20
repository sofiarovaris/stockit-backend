-- Your SQL goes here
CREATE TABLE user (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    rg VARCHAR(9) NOT NULL,
    cpf VARCHAR(11) NOT NULL,
    phone VARCHAR(11) NOT NULL,
    username VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    number INT NOT NULL,
    complement VARCHAR(255),
    address_id INT NOT NULL,
    FOREIGN KEY (address_id) REFERENCES address(id)
);