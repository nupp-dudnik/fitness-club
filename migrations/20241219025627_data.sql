CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

INSERT INTO subscriptions ("id", "access_level", "price", "period", "custom_period")
VALUES
(uuid_generate_v4(), 'Premium',   '4000', 'Yearly', NULL),
(uuid_generate_v4(), 'Family',    '2000', 'Yearly', NULL),
(uuid_generate_v4(), 'EarlyBird', '1600', 'Yearly', NULL),
(uuid_generate_v4(), 'Basic',     '1000', 'Yearly', NULL),
(uuid_generate_v4(), 'Premium',   '400', 'Monthly', NULL),
(uuid_generate_v4(), 'Family',    '200', 'Monthly', NULL),
(uuid_generate_v4(), 'EarlyBird', '160', 'Monthly', NULL),
(uuid_generate_v4(), 'Basic',     '100', 'Monthly', NULL),
(uuid_generate_v4(), 'Premium',   '200', 'Weekly', NULL),
(uuid_generate_v4(), 'Family',    '100', 'Weekly', NULL),
(uuid_generate_v4(), 'EarlyBird', '80',  'Weekly', NULL),
(uuid_generate_v4(), 'Basic',     '50',  'Weekly', NULL),
(uuid_generate_v4(), 'Premium',   '100', 'Custom', 'Every other week');

INSERT INTO locations ("id", "country", "city", "street", "working_hours")
VALUES
('c242642c-fbbc-4292-a9f9-9471821a9806', 'Ukraine', 'Poltava', 'Peremogi 15',   '6:00-22:00'),
('9a821296-c116-4f0b-ba05-6ee0ea026650', 'Ukraine', 'Kyiv',    'Shevchenko 25', '6:00-22:00'),
('ee8d15c9-6191-4ea3-a62b-b0e421b2a891', 'Ukraine', 'Poltava', 'Shevchenko 25', NULL);


INSERT INTO tools ("id", "name", "kind")
VALUES
('91247812-12fc-49c2-8ab7-6c30c1e3df53', 'Yoga Mat',  'Fitness'),
('0bd6497a-c4da-4b1d-a3c0-b74856a79589', 'Yoga Ball', 'Fitness'),
('deadbeef-07c0-447a-a721-254ac433e32e', 'Dumbbells', 'Strength');


INSERT INTO tools_at_location ("location", "tool", "condition", "description")
VALUES
('c242642c-fbbc-4292-a9f9-9471821a9806', '91247812-12fc-49c2-8ab7-6c30c1e3df53',  'prefect', NULL),
('9a821296-c116-4f0b-ba05-6ee0ea026650', '0bd6497a-c4da-4b1d-a3c0-b74856a79589',  'needs replacement', NULL),
('ee8d15c9-6191-4ea3-a62b-b0e421b2a891', 'deadbeef-07c0-447a-a721-254ac433e32e',  'good', NULL);

INSERT INTO tasks ("id", "name", "description")
VALUES
(uuid_generate_v4(), 'Training', 'Customer service'),
(uuid_generate_v4(), 'Managing', 'Equipment and customers'),
(uuid_generate_v4(), 'Human Recourses', 'Hiring staff'),
(uuid_generate_v4(), 'Cleaning', 'Equipment maintenance');
