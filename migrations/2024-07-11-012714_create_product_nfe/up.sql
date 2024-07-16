-- Your SQL goes here
CREATE TABLE product_nfe (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    calculation_basis DECIMAL(10, 2) NULL,
    icms_intern DECIMAL(10, 2) NULL,
    ipi DECIMAL(10, 2) NULL,
    origin VARCHAR(255) NULL,
    cest DECIMAL(10, 2) NULL,
    ncm DECIMAL(10, 2) NULL,
    csosn DECIMAL(10, 2) NULL,
    product_id INTEGER NOT NULL,
    FOREIGN KEY (product_id) REFERENCES product(id)
);