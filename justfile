default:
    just --list

expand:
    cargo expand -p diesel-json-derive-test

test:
    cargo build

readme:
    cargo readme > README.md

# -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
# postgres

data_dir := justfile_directory() / "postgres_data"
run_dir := justfile_directory() / "postgres_run"
port := "5433"
user := "postgres"

pg_init:
    initdb -D {{ data_dir }} -U {{ user }} -A trust
    sed -i "s/#port = 5432/port = {{ port }}/" {{ data_dir }}/postgresql.conf
    sed -i "s/#unix_socket_directories/unix_socket_directories/" {{ data_dir }}/postgresql.conf
    sed -i "s|/var/run/postgresql|{{ run_dir }}|" {{ data_dir }}/postgresql.conf
    sed -i "s|/run/postgresql|{{ run_dir }}|" {{ data_dir }}/postgresql.conf
    mkdir -p {{ run_dir }}

pg_clean:
    rm -rf {{ data_dir }} {{ run_dir }} postgres.log | true

pg_ctl *args="status":
    pg_ctl -D {{ data_dir }} -l postgres.log {{ args }}

pg_status:
    just pg_ctl status

pg_start:
    just pg_ctl start

pg_stop:
    just pg_ctl stop || true

pg_tryagain: pg_stop pg_clean pg_init pg_start

psql *args:
    psql --port {{ port }} --host localhost -d postgres -U {{ user }} {{ args }}
