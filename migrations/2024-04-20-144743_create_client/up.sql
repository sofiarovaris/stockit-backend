-- Your SQL goes here
CREATE TABLE client (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    phone VARCHAR(255) NOT NULL,
    cell VARCHAR(255) NOT NULL,
    cpf VARCHAR(255) NULL,
    cnpj VARCHAR(255) NULL,
    type VARCHAR(255) NOT NULL,
    company_name VARCHAR(255) NULL,
    rg VARCHAR(255) NULL,
    address_id INTEGER NOT NULL,
    FOREIGN KEY (address_id) REFERENCES address(id)
);