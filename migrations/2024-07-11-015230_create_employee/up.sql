-- Your SQL goes here
CREATE TABLE IF NOT EXISTS employee (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    admission_date DATE NULL,
    payment_date DATE NULL,
    salary DECIMAL(10, 2) NULL,
    comission DECIMAL(10, 2) NULL,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES user(id)
);