Crate - Compile-time safety - Query interface - Async
tokio-postgres - No - SQL - Yes
sqlx Yes SQL Yes
diesel Yes DSL NO
we will use sqlx: its asynchronous support sim- plifies the integration with actix-web without forcing us to compromise on compile-time guarantees. It also limits the API surface that we have to cover and become proficient with thanks to its usage of raw SQL for queries.
# docker pull postgres
# see scripts/init_db.sh

Let’s make it executable:
# chmod +x scripts/init_db.sh
We can then launch Postgres with
# ./scripts/init_db.sh
If you run docker ps you should see something along the lines of
IMAGE            PORTS                   STATUS
postgres   127.0.0.1:5432->5432/tcp   Up 12 seconds   [...]

cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres
# Assuming you used the default parameters to launch Postgres in Docker!
export DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/newsletter
sqlx migrate add create_subscriptions_table