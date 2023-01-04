# Deploying

## Running in a Kubernetes cluster

TODO: Write

# For developers

## Set up development environment

### 1. Create user config

This config will be picked up automatically
```bash
echo "DB_USER=$(whoami)
DB_NAME=samling
SECRET=abc123
LOG_LEVEL=info
CORS_ALLOWED_ORIGINS=http://localhost:3000,http://127.0.0.1:3000
CLOUDFLARE_ACCOUNT_ID=abc
CLOUDFLARE_TOKEN=123
SAMLING_PRETTY_RESPONSES=true" > .env
```

### 2. Create database and migrate

```bash
createdb samling
cargo run migrate
```

### 3. Create user

Export the user ID as an environment variable, we'll use it later. USER_ID should be `1` as in this example, if the database is clean. We'll also store SUPERUSER_ID for CLI access.

```bash
cargo run users create --name 'My Name' --email my@email.com --password MyPassword
export USER_ID=1
echo SUPERUSER_ID=$USER_ID >> .env
```

### 4. Create organization
```bash
cargo run organizations create 'Company Name'
export ORGANIZATION_ID=1
```

### 5. Associate all roles to superuser

NOTE: `ORGANIZATION_ID` might be something different than `1`.

```bash
cargo run users associate-roles --all $USER_ID $ORGANIZATION_ID
```

### 6. Start the API server
```bash
cargo run serve
```

### 7. (Optional) Store API token for conveniently making API calls

NOTE: The stored API token will have to be updated every 7 days for the `api` command to work.

```bash
echo SAMLING_TOKEN=$(cargo run generate-user-token $USER_ID) >> .env
```


Pro tip! You can use [cargo-watch](https://crates.io/crates/cargo-watch) to have the server restart every time the code changes.
```bash
cargo watch -- cargo run serve
```

### 8. Start the frontend
This is easy! Just do:
```bash
cd ui
npm install
npm start
```

## Creating data via API
Currently the only way to create categories, prices, styles etc is via the API.

For example, to create a category called T-Shirts:

```bash
echo '{"name": {"en": "T-shirts"}}' | samling api -d - PUT 1/categories/external_id:BC-TShirts
```

A style:
```bash
echo '{"number": "A12345", "name": {"en": "Cool t-shirt"}}' | samling api -d - PUT 1/styles/external_id:BC-A12345
```

A color, associated with the above style:
```bash
echo '{"number": "HBlue", "name": {"en": "Hazy Blue"}, "style": {"external_id": "BC-A12345"}}' | samling api -d - PUT 1/colors/external_id:BC-A12345-HBlue
```

A size, associated with the above color:
```bash
echo '{"number": "XS", "name": {"en": "Extra small"}, "position": 1, "color": {"external_id": "BC-A12345-HBlue"}}' | samling api -d - PUT 1/sizes/external_id:BC-A12345-HBlue-XS
```

Notice how we're doing `PUT` requests here, with an External ID specified in the URL. This would be the typical way of syncing data from your ERP system.
