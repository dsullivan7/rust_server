#! /bin/bash

echo "DB_DROP: ${DB_DROP}"

if [[ ${DB_DROP} = 'yes' ]]; then
  echo "seeding the database ${DB_NAME}"
  for FILE in ${SEEDER_DIR}/*; do
    PGPASSWORD=${DB_PASSWORD} psql -h ${DB_HOST} -p ${DB_PORT} -U ${DB_USER} ${DB_NAME} -f $FILE;
  done
fi
