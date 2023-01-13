create table securities (
  security_id uuid primary key unique not null default (uuid_generate_v4()),
  symbol text not null,
  name text not null,
  description text,
  beta double precision,
  created_at timestamptz not null default (now()),
  updated_at timestamptz not null default (now())
);
