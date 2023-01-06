create table assets (
  asset_id uuid primary key unique not null default (uuid_generate_v4()),
  type text,
  value integer,
  expires_at timestamptz,
  created_at timestamptz not null default (now()),
  updated_at timestamptz not null default (now())
);
