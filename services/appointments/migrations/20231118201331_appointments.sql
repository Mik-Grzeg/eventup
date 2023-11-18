CREATE TABLE IF NOT EXISTS services (
  service_id UUID PRIMARY KEY,
  company_id UUID NOT NULL,
  name TEXT NOT NULL,
  duration interval NOT NULL,
  price MONEY NOT NULL
);

CREATE TABLE IF NOT EXISTS appointments (
  appointment_id UUID NOT NULL,
  company_id UUID NOT NULL,
  service_id UUID REFERENCES services (service_id),

  -- metadata about the edit history
  created_at TIMESTAMP,
  updated_at TIMESTAMP,

  -- information about client and employee
  client_id UUID NOT NULL,
  employee_id UUID NOT NULL,
  client_name TEXT,

  -- appointment time
  start_time TIMESTAMP,
  end_time TIMESTAMP,

  -- price related columns
  price_expected MONEY,
  price_final MONEY,
  discount DECIMAL,

  -- tells whether the service has been canceled/provided
  canceled BOOLEAN, 
  cancellation_reason TEXT,
  provided BOOLEAN, 

  PRIMARY KEY(appointment_id, company_id)
);
