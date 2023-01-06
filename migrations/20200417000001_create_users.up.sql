create table users (
  user_id uuid primary key unique not null default (uuid_generate_v4()),
  first_name text,
  last_name text,
  auth0_id text unique,
  phone_number text,
  email text,
  address text,
  city text,
  state text,
  postal_code text,
  date_of_birth date,
  ssn text,
  dwolla_customer_id text unique,
  created_at timestamptz not null default (now()),
  updated_at timestamptz not null default (now())
);
