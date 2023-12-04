CREATE TABLE IF NOT EXISTS services (
  service_id UUID PRIMARY KEY,
  -- company_id UUID NOT NULL,
  name TEXT NOT NULL,
  description TEXT,
  duration_in_sec INT NOT NULL,
  price FLOAT4 NOT NULL,

  -- metadata about the edit history
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

  active BOOLEAN DEFAULT true
);

CREATE TABLE IF NOT EXISTS appointments (
  appointment_id UUID PRIMARY KEY,
  -- company_id UUID NOT NULL,
  service_id UUID REFERENCES services (service_id),

  -- metadata about the edit history
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

  -- information about client and employee
  client_id UUID NOT NULL,
  employee_id UUID NOT NULL,
  client_name TEXT NOT NULL,

  -- appointment time
  start_time TIMESTAMP WITH TIME ZONE NOT NULL,
  end_time TIMESTAMP WITH TIME ZONE NOT NULL,

  -- price related columns
  price_expected FLOAT4 NOT NULL,
  price_final FLOAT4,
  discount FLOAT4,

  -- tells whether the service has been canceled/provided
  canceled BOOLEAN NOT NULL DEFAULT false, 
  cancellation_reason TEXT,
  served BOOLEAN NOT NULL DEFAULT false,
  UNIQUE(start_time, end_time, employee_id)
);

CREATE TABLE IF NOT EXISTS employee_schedules (
  schedule_id UUID PRIMARY KEY,
  employee_id UUID NOT NULL,
  service_id UUID NOT NULL,
  start_shift TIME NOT NULL, 
  end_shift TIME NOT NULL,
  UNIQUE(employee_id, service_id)
);
