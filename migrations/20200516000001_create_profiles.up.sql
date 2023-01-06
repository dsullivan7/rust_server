create table profiles (
  profile_id uuid primary key unique not null default (uuid_generate_v4()),
  username text not null,
  password text not null,
  type text not null,
  created_at timestamptz not null default (now()),
  updated_at timestamptz not null default (now())
);
