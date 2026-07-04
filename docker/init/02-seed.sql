INSERT INTO users (name, surname, birth_year, email) VALUES
('John', 'Smith', 1990, 'john.smith@test.com'),
('Adam', 'Newman', 1980, 'adam.newman@test.com'),
('Anna', 'Drake', 1973, 'anna.drake@test.com'),
('John', 'Deep', 1970, 'john.deep@test.com');

INSERT INTO orders (user_id, product_name, quantity, amount) VALUES
(1, 'Wireless Mouse', 2, 49.98),
(1, 'Mechanical Keyboard', 1, 129.0),
(2, 'USB-C Hub', 1, 39.5),
(2, '27" Monitor', 1, 329.99),
(3, 'Noise-Cancelling Headphones', 1, 249.0),
(3, 'Webcam 1080p', 3, 89.97),
(4, 'Standing Desk', 1, 459.0),
(4, 'Ergonomic Chair', 1, 319.5),
(1, 'Laptop Stand', 2, 59.98),
(2, 'External SSD 1TB', 1, 119.0);