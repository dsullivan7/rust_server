create table posts (
  post_id uuid primary key unique not null default (uuid_generate_v4()),
  group_id uuid references groups on delete set null on update cascade,
  name text,
  message text,
  url text,
  created_at timestamptz not null default (now()),
  updated_at timestamptz not null default (now())
);
