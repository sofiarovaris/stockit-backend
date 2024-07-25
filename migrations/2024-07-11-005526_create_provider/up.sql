-- Your SQL goes here
CREATE TABLE IF NOT EXISTS provider (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(255) NULL,
    company_name VARCHAR(255) NULL,
    state_registration VARCHAR(255) NULL,
    email VARCHAR(255) NULL,
    phone VARCHAR(255) NULL,
    cnpj VARCHAR(255) NULL,
    bank_reference VARCHAR(255) NULL,
    address_id INTEGER NULL,
    number INTEGER NULL,
    complement VARCHAR(255) NULL,
    FOREIGN KEY (address_id) REFERENCES address(id)
);