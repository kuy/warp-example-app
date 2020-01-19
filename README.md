## Web app example: warp + diesel + tera

### Requirements

- Rust (1.40 or later)
- Diesel CLI (1.4.0)
- Docker (2.1 or later)

### Getting Started

1. Build and start MySQL in Docker

```
cd misc/db
./build
./run
```

2. Run database migration

```
diesel migration run
```

3. Insert data (Optional)

You can use `misc/db/data.sql` to prepare data.

```
mysql --host="127.0.0.1" --port="3306" \
  --database="warpexample" --user="warpexample" \
  --password="warpexample" < "misc/db/data.sql"
```

4. Put `.env` copied from `.env.example`

No need to change except you changed MySQL config.

5. Launch application

```
cargo run
```

6. Open endpoint your browser

- List of tasks: `http://localhost:3030`
- Greetings: `http://localhost:3030/hello/yourname`
