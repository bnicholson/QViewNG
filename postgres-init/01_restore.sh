# postgres-init/01_restore.sh
#!/bin/bash
set -e

echo "Restoring backup..."
pg_restore \
  --username="$POSTGRES_USER" \
  --dbname="$POSTGRES_DB" \
  --no-owner \
  --no-privileges \
  /docker-entrypoint-initdb.d/init_seed_data.backup

echo "Restore complete."