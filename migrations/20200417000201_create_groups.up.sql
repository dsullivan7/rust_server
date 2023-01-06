create table groups (
  group_id uuid primary key unique not null default (uuid_generate_v4()),
  name text,
  api_client_key text,
  api_client_secret text,
  created_at timestamptz not null default (now()),
  updated_at timestamptz not null default (now())
);
