Crate          | Compile-time safety | Query interface | Async

---------------------------------------------------------------
tokio-postgres | No                  | SQL             | Yes
sqlx           | Yes                 | SQL             | Yes
diesel         | Yes                 | DSL             | NO
---------------------------------------------------------------
we will use sqlx: its asynchronous support sim- plifies the integration with actix-web without forcing us to compromise on compile-time guarantees. It also limits the API surface that we have to cover and become proficient with thanks to its usage of raw SQL for queries.

# docker pull postgres -> get image
# see scripts/init_db.sh -> setups config postgres

Postgres’ default settings

Let’s make it executable:
# chmod +x scripts/init_db.sh

We can then launch Postgres with
# ./scripts/init_db.sh
If you run docker ps you should see something along the lines of
IMAGE            PORTS                   STATUS
postgres   127.0.0.1:5432->5432/tcp   Up 12 seconds   [...]

# Adding A Migration
Database Migrations To store our subscribers details we need to create our first table. To add a new table to our database we need to change its schema - this is commonly referred to as a database migration.
cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres

# Assuming you used the default parameters to launch Postgres in Docker!
export DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/newsletter
sqlx migrate add create_subscriptions_table

# {timestamp}_create_subscriptions_table.sql
Under migrations you should already have one file called {timestamp}_create_subscriptions_table.sql
- this is where we have to write the SQL code for our first migration. Let’s quickly sketch the query we need:

# Running Migrations
start docker
in ./scripts/init_db.sh with sqlx migrate run
then => SKIP_DOCKER=true ./scripts/init_db.sh

on production -> export DATABASE_URL flow config on production.yaml

# configuration.yaml *
We want to read our application settings from a configuration file named configuration:

Just like sqlx-cli commands, it relies on the DATABASE_URL environment variable to know where to find the database.
We could export DATABASE_URL manually, but we would then run in the same issue every time we boot our machine and start working on this project. 
Let’s take the advice of sqlx’s authors - we’ll add a top-level .env file
configuration.yaml can be used to alter the runtime behaviour of the application after it has been compiled, 
while .env is only relevant for our development process, build and test steps.

-> configuration.yaml past to configuration for env .yaml 

# create image
docker build --tag zero2prod --file Dockerfile .

# docker run -p 8000:8000 zero2prod
We are using 127.0.0.1 as our host in address  which is not seen as local by our Docker image,
we will keep using 127.0.0.1 for our local development and set it to 0.0.0.0 in our Docker images.
setup local development and production

#  cargo clippy -- -D warnings