create table invoices (
  invoice_id uuid primary key unique not null default (uuid_generate_v4()),
  bank_account_id uuid references bank_accounts on delete set null on update cascade,
  created_at timestamptz not null default (now()),
  updated_at timestamptz not null default (now())
);
