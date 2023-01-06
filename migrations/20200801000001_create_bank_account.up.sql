create table bank_accounts (
  bank_account_id uuid primary key unique not null default (uuid_generate_v4()),
  user_id uuid references users on delete set null on update cascade,
  name text,
  plaid_access_token text,
  plaid_account_id text,
  dwolla_funding_source_id text,
  alpaca_ach_relationship_id text,
  master_account boolean not null default false,
  created_at timestamptz not null default (now()),
  updated_at timestamptz not null default (now())
);
