To run sqlx migrations, install sqlx-cli cargo package with: `cargo install sqlx-cli`

Command to create the database: `sqlx database create`

Command to create migrations: `sqlx migrate add -r <name>`

Command to run migrations: `sqlx migrate run`

Run: `cargo sqlx prepare` to cache the queries for macros.

Generate an RSA key pair via:

```bash
openssl genpkey -algorithm RSA -out private.pem -pkeyopt rsa_keygen_bits:2048
```

```bash
openssl rsa -pubout -in private.pem -out public.pem
```

Store the keys in `keys` folder at the root of this project.