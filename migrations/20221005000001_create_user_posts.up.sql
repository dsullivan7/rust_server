create table user_posts (
  user_post_id uuid primary key unique not null default (uuid_generate_v4()),
  user_id uuid references users on delete set null on update cascade,
  post_id uuid references posts on delete set null on update cascade,
  created_at timestamptz not null default (now()),
  updated_at timestamptz not null default (now())
);
