CREATE TYPE user_roles AS ENUM ('admin', 'regular', 'employee');
CREATE TABLE IF NOT EXISTS user_log_infos (
  user_id UUID PRIMARY KEY,
  -- in case we wanted to make it multi tenant sas
  -- group_id INT,
  email text UNIQUE NOT NULL,
  password_hashed text,
  password_salt VARCHAR(50), role user_roles NOT NULL DEFAULT 'regular' -- CONSTRAINT fk_group
  --   FOREIGN KEY(group_id)
  --     REFERENCES groups(group_id)
  -- in case we wanted to migrate to other hashing algorithm, we need to know what was the previous one 
  -- password_hash_algorithm  
);

CREATE TABLE IF NOT EXISTS user_accounts (
  user_id UUID PRIMARY KEY REFERENCES user_log_infos (user_id),
  phone_number text UNIQUE NOT NULL,
  first_name VARCHAR(255),
  last_name VARCHAR(255),
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

