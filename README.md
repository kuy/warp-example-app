## Stack

- [warp](https://github.com/seanmonstar/warp): Web Application Framework
- [diesel](https://github.com/diesel-rs/diesel): ORM (MySQL)
- [tera](https://github.com/Keats/tera): Template Engine

## Implemented [Filters](https://docs.rs/warp/latest/warp/trait.Filter.html)

- [`create_template_filter`](https://github.com/kuy/warp-example-app/blob/master/src/main.rs#L17-L20): Prepare tera instance
- [`create_database_filter`](https://github.com/kuy/warp-example-app/blob/master/src/main.rs#L24-L29): Prepare connection pool of MySQL

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (1.40 or later)
- [Diesel CLI](http://diesel.rs/guides/getting-started/) (1.4.0)
- [Docker](https://www.docker.com/get-started) (2.1 or later)

## Getting Started

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

## License

MIT

## Author

Yuki Kodama / [@kuy](https://twitter.com/kuy)
