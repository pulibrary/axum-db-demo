### Setup

1. Install devbox
1. `devbox shell`
1. `devbox run migrate`
1. `devbox run dev`
1. In your browser, go to `localhost:${AXUM_PORT:-3333}/diesel` to see a list
   of dogs taken from the db
1. `psql -p ${PGPORT:-5432} axum-demo` to mess with the db directly

If you are already using port 5432 or 3333, you can export PGPORT or AXUM_PORT
to another port number, e.g. `export PGPORT=1234` or export `AXUM_PORT=2345`
