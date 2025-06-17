# Code Along For [Rust Axum Full Course](https://www.youtube.com/watch?v=XZtlD_m59sM&t=22s) by **Jeremy Chone**

---

## Dynamically Watching and Recompiling on Changes to src

You can use cargo watch for this

- First you need to install cargo watch, it is on lifesupport so you may need to use bacon or other
  libraries for this

```bash
cargo install cargo-watch
```

- To keep check on the server you can use

```bash
cargo watch -q -c -w src/ -x run
```

- To keep check on tests and outputs you can use

```bash
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```

-q -> quiet
-c -> clear
-w src/ -> Watch only src/ folder
-x run -> Execute run

## Sea-ORM Migration setup

### Installing Sea-ORM-CLI for managing migrations

```bash
cargo install sea-orm-cli
sea-orm-cli migrate init
```

### Generating a migration

```bash
sea-orm-cli migrate generate create_user_table
```

This will create migration with default post table.

### Running a migration

```bash
sea-orm-cli migrate up
```

## Sea-ORM Entity Generation

If `DATABASE_URL` is defined in .env file. Then you can auto generate entities using

```bash
sea-orm-cli generate entity -o entity/src
```

Otherwise

```bash
sea-orm-cli generate entity -u protocol:://user:password@endpoint_url/database -o entity/src
```
