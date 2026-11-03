### Setup

1. Install devbox
1. `devbox shell`
1. `source bin/setup_dev_environment`
1. `diesel migration run`
1. `devbox run dev`
1. In your browser, go to localhost:[PORT]/diesel to see a list of dogs taken from the db
1. Go to `psql -p ${PGPORT:-5422} axum-demo` to mess with the db directly
1. Check the application in your browser
