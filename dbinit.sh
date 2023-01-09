#! /bin/bash

echo "DB_DROP: ${DB_DROP}"

if [[ ${DB_DROP} = 'yes' ]]; then
  echo "dropping database ${DB_NAME}"
  PGPASSWORD=${DB_PASSWORD} psql -h ${DB_HOST} -p ${DB_PORT} -U ${DB_USER} ${DB_NAME} -c "select pg_terminate_backend(pid) from pg_stat_activity where pid <> pg_backend_pid() and datname='${DB_NAME}';"
  PGPASSWORD=${DB_PASSWORD} dropdb -h ${DB_HOST} -p ${DB_PORT} -U ${DB_USER} ${DB_NAME}

  echo "creating database ${DB_NAME}"
  PGPASSWORD=${DB_PASSWORD} createdb -h ${DB_HOST} -p ${DB_PORT} -U ${DB_USER} ${DB_NAME}
fi
