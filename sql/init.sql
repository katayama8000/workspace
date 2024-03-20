CREATE TABLE IF NOT EXISTS Users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    gender VARCHAR(10)
);

CREATE TABLE IF NOT EXISTS MonthlyBills (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES Users(id),
    total_amount DECIMAL(10, 2),
    status VARCHAR(20),
    month VARCHAR(20)
);

CREATE TABLE IF NOT EXISTS BillDetails (
    id SERIAL PRIMARY KEY,
    bill_id INT REFERENCES MonthlyBills(id),
    name VARCHAR(255) NOT NULL,
    price DECIMAL(10, 2),
    quantity INT
);

CREATE TABLE IF NOT EXISTS RecurringBills (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES Users(id),
    name VARCHAR(255) NOT NULL,
    amount DECIMAL(10, 2),
    active BOOLEAN DEFAULT TRUE
);

INSERT INTO
    Users (name, email, gender)
VALUES
    ('John Doe', 'john.doe@example.com', 'Male');

INSERT INTO
    MonthlyBills (user_id, total_amount, status, month)
VALUES
    (1, 100, 'completed', 'June');

INSERT INTO
    BillDetails (bill_id, name, price, quantity)
VALUES
    (1, 'Electricity', 50, 1),
    (1, 'Water', 30, 1),
    (1, 'Internet', 20, 1);

INSERT INTO
    RecurringBills (user_id, name, amount)
VALUES
    (1, 'Rent', 500),
    (1, 'Groceries', 200);