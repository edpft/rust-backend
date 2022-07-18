-- Create Clients Table
CREATE TABLE IF NOT EXISTS clients(
   id BIGSERIAL PRIMARY KEY,
   first_name TEXT NOT NULL,
   last_name TEXT NOT NULL,
   email_address TEXT NOT NULL UNIQUE,
   telephone_number TEXT NOT NULL UNIQUE,
   added_at timestamptz NOT NULL DEFAULT NOW()
);