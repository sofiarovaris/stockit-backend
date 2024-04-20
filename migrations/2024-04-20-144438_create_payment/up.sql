-- Your SQL goes here
CREATE TABLE payment (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    amount DECIMAL(10, 2) NOT NULL,
    month DECIMAL(10, 2) NOT NULL,
    year INTEGER NOT NULL,
    employee_id INTEGER NOT NULL,
    FOREIGN KEY (employee_id) REFERENCES employee(id)
);