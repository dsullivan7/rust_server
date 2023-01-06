create table items (
  item_id uuid primary key unique not null default (uuid_generate_v4()),
  invoice_id uuid references invoices on delete cascade on update cascade,
  name text not null,
  price numeric not null,
  created_at timestamptz not null default (now()),
  updated_at timestamptz not null default (now())
);
