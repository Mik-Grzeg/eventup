-- Add migration script here
INSERT INTO user_log_infos (user_id, email, password_hashed, password_salt) VALUES ('b9ee058b-3143-4176-851b-a60cde9d06eb', 'jjjj@doe.com', '$2y$12$vRwKdyG6eawZEllxkMD4T.SJTehGM5/.qaQHqYgV7qxkxbN03uESO', '\\xc53c8c7f423c81cc9b1a79f398e17a54');
INSERT INTO user_accounts (user_id, phone_number, created_at, updated_at) VALUES ('b9ee058b-3143-4176-851b-a60cde9d06eb', '+48123457789', '2023-11-29 23:10:16.305355+00', '2023-11-30 12:17:01.102156+00');
