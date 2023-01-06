create table portfolios (
  portfolio_id uuid primary key unique not null default (uuid_generate_v4()),
  user_id uuid references users on delete set null on update cascade,
  risk integer not null default 0,
  created_at timestamptz not null default (now()),
  updated_at timestamptz not null default (now())
);
