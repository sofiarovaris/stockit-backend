-- Your SQL goes here
CREATE TABLE IF NOT EXISTS product (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(255) NULL,
    code VARCHAR(255) NULL,
    observations TEXT NULL,
    gross_weight DECIMAL(10, 2) NULL,
    net_weight DECIMAL(10, 2) NULL,
    batch_number VARCHAR(255) NULL,
    current_quantity INTEGER NULL,
    provider_id INTEGER NOT NULL,
    producer_id INTEGER NOT NULL,
    unit_id INTEGER NOT NULL,
    FOREIGN KEY (provider_id) REFERENCES provider(id),
    FOREIGN KEY (unit_id) REFERENCES unit(id),
    FOREIGN KEY (producer_id) REFERENCES producer(id)
);