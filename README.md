# rust-axum-sqlx-redis-ws-template
Rust+Axum+Sqlx+Redis+Postgres Web Service Template

## Installation

Start by cloning the repository and navigating into it:
```bash
git clone https://github.com/softwaremill/rust-axum-sqlx-redis-ws-template
cd rust-axum-sqlx-redis-ws-template
cargo run
```

You should have a compiling project with the following panic message:
```
   Compiling bb8-redis v0.17.0
   Compiling rust-axum-sqlx-redis-ws-template v0.1.0 (/home/lmx/isima/zz2/projet/exemple)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3m 03s
     Running `target/debug/rust-axum-sqlx-redis-ws-template`
thread 'main' panicked at src/config.rs:9:58:
DATABASE_URL must be set: NotPresent
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

You now need to create a `.env` file using this template:
```bash
cat <<EOF > .env
DATABASE_URL=postgres://myuser:mypassword@localhost/mydb
CACHE_URL=redis://localhost:6379
EOF
```

Make sure to replace `myuser` and `mypassword` with your database credentials. If you do not have a database set up, follow this guide:

### Install Redis (for caching)
```bash
sudo pacman -S redis
sudo systemctl start redis
sudo systemctl enable redis # Auto-start Redis on boot
redis-cli ping # Check if Redis is running
```

### Install PostgreSQL
```bash
sudo pacman -S postgresql
sudo -iu postgres initdb -D /var/lib/postgres/data # Initialize database
sudo systemctl start postgresql
sudo systemctl enable postgresql # Auto-start PostgreSQL on boot
```

### Create a Database and User
```bash
sudo -iu postgres psql # Open PostgreSQL as superuser
```

```sql
SELECT usename FROM pg_user; -- Show existing users
CREATE USER myuser WITH PASSWORD 'mypassword'; -- Create a new user

\l -- List databases
CREATE DATABASE mydb OWNER myuser; -- Create a database

\c mydb -- Connect to the database
SELECT nspname, pg_catalog.pg_get_userbyid(nspowner) FROM pg_catalog.pg_namespace WHERE nspname = 'public'; -- Check database permissions
ALTER SCHEMA public OWNER TO myuser; -- Assign ownership to myuser

ALTER USER myuser WITH SUPERUSER; -- Grant superuser privileges (optional)

exit
```

### Run the Application
You can now restart the application:
```bash
cargo run
```

You should see:
```
2025-02-08T13:12:59.819320Z  INFO rust_axum_sqlx_redis_ws_template: Connecting to pg: postgres://myuser:mypassword@localhost/mydb
2025-02-08T13:12:59.819393Z  INFO rust_axum_sqlx_redis_ws_template: Connecting to cache: redis://localhost:6379
2025-02-08T13:12:59.938547Z DEBUG rust_axum_sqlx_redis_ws_template: listening on 127.0.0.1:3000
```

### Test the API
Et voila ! You can now visit http://127.0.0.1:3000/swagger-ui/ to interact with the API.
