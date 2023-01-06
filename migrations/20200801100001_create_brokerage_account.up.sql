create table brokerage_accounts (
  brokerage_account_id uuid primary key unique not null default (uuid_generate_v4()),
  user_id uuid references users on delete set null on update cascade,
  alpaca_account_id text,
  created_at timestamptz not null default (now()),
  updated_at timestamptz not null default (now())
);
