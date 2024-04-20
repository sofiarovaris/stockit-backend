-- Your SQL goes here
CREATE TABLE provider (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    phone VARCHAR(255) NOT NULL,
    cnpj VARCHAR(255) NULL,
    company_name VARCHAR(255) NULL,
    state_registration VARCHAR(255) NULL,
    address_id INTEGER NOT NULL,
    bank_reference VARCHAR(255) NULL,
    FOREIGN KEY (address_id) REFERENCES address(id)
);