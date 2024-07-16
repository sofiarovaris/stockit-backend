-- Your SQL goes here
CREATE TABLE client_user (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(255),
    email VARCHAR(255),
    phone VARCHAR(255),
    state_registration VARCHAR(255),
    cpf VARCHAR(255),
    cnpj VARCHAR(255),
    client_type VARCHAR(255),
    company_name VARCHAR(255),
    rg VARCHAR(255),
    address_id INTEGER NULL,
    number INTEGER,
    complement VARCHAR(255),
    FOREIGN KEY (address_id) REFERENCES address(id)
);