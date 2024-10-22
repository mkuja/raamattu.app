#!/bin/bash

# Ensure environment variables are set
export PGPASSWORD=$POSTGRES_PASSWORD

# Check if the 'books' table exists
NEEDS_MIGRATION=$(psql -h postgres -U postgres -d "$POSTGRES_DB" -t -c "SELECT EXISTS(SELECT 1 FROM information_schema.tables WHERE table_name='books');")

if [[ "$NEEDS_MIGRATION" =~ "f" ]]; then
    echo "Setting up initial database from dump."
    tar -xJvf raamattu.pgsql.tar.xz
    psql -h postgres -U postgres -d "$POSTGRES_DB" -f raamattu.pgsql
    echo "Database schema and initial data loaded."
else
    echo "Database schema already exists. Skipping migration."
fi

echo "Migration container will now exit."