create table orders (
  order_id uuid primary key unique not null default (uuid_generate_v4()),
  parent_order_id uuid references orders on delete set null on update cascade,
  matching_order_id uuid references orders on delete set null on update cascade,
  portfolio_id uuid references portfolios on delete set null on update cascade,
  user_id uuid references users on delete set null on update cascade,
  symbol text,
  side text,
  status text,
  amount integer,
  alpaca_order_id text,
  completed_at timestamptz,
  created_at timestamptz not null default (now()),
  updated_at timestamptz not null default (now())
);
