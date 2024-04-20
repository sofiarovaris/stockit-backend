-- Your SQL goes here
CREATE TABLE salary_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    salary DECIMAL(10, 2) NOT NULL,
    date DATE NOT NULL,
    employee_id INTEGER NOT NULL,
    FOREIGN KEY (employee_id) REFERENCES employee(id)
);