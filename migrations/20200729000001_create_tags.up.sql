create table tags (
  tag_id uuid primary key unique not null default (uuid_generate_v4()),
  name text,
  created_at timestamptz not null default (now()),
  updated_at timestamptz not null default (now())
);
