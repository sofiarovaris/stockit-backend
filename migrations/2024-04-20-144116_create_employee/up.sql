-- Your SQL goes here
CREATE TABLE employee (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    admission_date DATE NOT NULL,
    payment_date DATE NOT NULL,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES user(id)
);