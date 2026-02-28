# Leveling Education Framework
Better navigation for [HBO-I Domeinbeschrijving](https://www.hbo-i.nl/publicaties-domeinbeschrijving/) and Open-ICT Vaardigheden.

Built with [Rust](https://www.rust-lang.org/), [Rocket](https://rocket.rs/) and [Tidos](https://crates.io/crates/tidos).

## Running locally

```bash
cargo run
```

## Updating content

Content lives in `/app/data` as JSON files. Edit the relevant file and redeploy to update vaardigheden, HBO-I competenties, or beroepsproducten.

## Environment variables

| Name | Example | Description |
| --- | --- | --- |
| `HOSTS` | `localhost, lef.open-ict.hu.nl` | Comma-separated list of hosts that Caddy generates HTTPS certificates for |

Copy `.env-example` to `.env` and fill in the values before deploying.

## Running in production

1. Copy `.env-example` to `.env` and edit the values.
2. Create the shared Docker network: `docker network create caddy`
3. Start Caddy: `docker compose -f docker-compose.caddy.yml --env-file .env up -d`
4. Start the application: `docker compose --env-file .env up -d`

Watchtower automatically pulls and restarts the container when a new image is published to Docker Hub.

## Contributing

We love your input! Whether it's:

- Reporting a bug
- Discussing the current state of the code
- Submitting a fix
- Proposing new features

We use GitHub to host code, track issues, and accept pull requests via [GitHub Flow](https://guides.github.com/introduction/flow/index.html):

1. Fork the repo and create your branch from `main`.
2. Make your changes.
3. If you've changed the API, update this documentation.
4. Open a pull request and target the `main` branch.

Report bugs via [GitHub Issues](https://github.com/spark-156/leveling-education-framework/issues).

## License

By contributing, you agree that your contributions will not be licensed and you lose all rights to your code.

## API

The API is available under `/api/v1`.

### `GET /api/v1/hboi`

Returns all HBO-I competenties.

**Response**
```json
{
  "<architectuurlaag> <activiteit>": {
    "1": { "title": "string", "info": "string | null" },
    "2": { "title": "string", "info": "string | null" },
    "3": { "title": "string", "info": "string | null" },
    "4": { "title": "string", "info": "string | null" }
  }
}
```

### `GET /api/v1/vaardigheden`

Returns all Open-ICT vaardigheden.

**Response**
```json
{
  "<vaardigheid>": {
    "1": { "title": "string", "info": "string | null" },
    "2": { "title": "string", "info": "string | null" },
    "3": { "title": "string", "info": "string | null" },
    "4": { "title": "string", "info": "string | null" }
  }
}
```

### `GET /api/v1/beroepsproducten`

Returns all beroepsproducten voorbeelden.

**Response**
```json
[
  {
    "architecture_layer": "string",
    "activity": "string",
    "guild": "string",
    "title": "string"
  }
]
```
