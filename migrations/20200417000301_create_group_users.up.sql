create table group_users (
  group_user_id uuid primary key unique not null default (uuid_generate_v4()),
  user_id uuid references users on delete set null on update cascade,
  group_id uuid references groups on delete set null on update cascade,
  created_at timestamptz not null default (now()),
  updated_at timestamptz not null default (now())
);
