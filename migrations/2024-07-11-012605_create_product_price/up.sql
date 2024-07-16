-- Your SQL goes here
CREATE TABLE product_price (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    price_cost DECIMAL(10, 2) NULL,
    sale_price DECIMAL(10, 2) NULL,
    profit_margin DECIMAL(10, 2) NULL,
    product_id INTEGER NOT NULL,
    FOREIGN KEY (product_id) REFERENCES product(id)
);