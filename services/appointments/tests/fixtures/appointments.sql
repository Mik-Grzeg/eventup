-- Add migration script here
INSERT INTO appointments (
  appointment_id,
  service_id,
  created_at,
  updated_at,
  client_id,
  client_name,
  employee_id,
  start_time,
  end_time,
  price_expected,
  price_final,
  discount,
  canceled,
  cancellation_reason,
  served)
VALUES 
('4875cb35-fb88-4419-acc0-d6b7348cf95b', '00000000-0000-0000-0000-000000000001', '2023-12-02 07:00:01.000001+00', '2023-12-02 07:00:10.000000+00', 'b9ee058b-3143-4176-851b-a60cde9d06eb', 'john', '00000000-0000-0000-0000-00000000000a', '2023-12-05 10:00:00+00', '2023-12-05 11:00:00.000000+00', 120.0, 110.0, 10.0, false, NULL, false),
('7b5a2ad4-6afd-4c17-bb1d-f8f3b129d0e8', '00000000-0000-0000-0000-000000000001', '2023-12-02 08:00:01.000001+00', '2023-12-02 08:00:10.000000+00', 'b9ee058b-3143-4176-851b-a60cde9d06eb', 'johnnys brother', '00000000-0000-0000-0000-00000000000b', '2023-12-05 12:00:00+00', '2023-12-05 13:00:00.000000+00', 120.0, 110.0, 10.0, false, NULL, false);
