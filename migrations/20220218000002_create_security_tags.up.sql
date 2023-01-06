create table security_tags (
  security_tag_id uuid primary key unique not null default (uuid_generate_v4()),
  security_id uuid references securities on delete cascade on update cascade,
  tag_id uuid references tags on delete cascade on update cascade,
  created_at timestamptz not null default (now()),
  updated_at timestamptz not null default (now())
);
