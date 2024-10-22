# This image is supposed to apply initial schema and bible translations to the database.
# It is invoked from the compose.yml by docker compose.
FROM postgres:17

WORKDIR /migrate
COPY db/raamattu.pgsql.tar.xz .
COPY db/initial_migration.sh .
RUN chmod +x initial_migration.sh
CMD ["./initial_migration.sh"]
