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
| `HOSTS` | `localhost, lef.hu.nl` | Comma-separated list of hosts that Caddy generates HTTPS certificates for |

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

The data API is versioned under `/api/v1` and `/api/v2`. Every dataset is also available as Markdown under `/llms/...` (same path, same query parameters, `text/markdown` instead of JSON) for consumption by AI agents and coaches — see `/llms.txt` for a full index and `/llms-full.txt` for the complete reference including teaching philosophy.

A query parameter that doesn't apply to a given endpoint is ignored rather than erroring. Filters that do apply are combined with AND; omitting a filter returns all values for it.

### `GET /api/v2/vaardigheden`

Returns all Open-ICT vaardigheden. Markdown equivalent: `GET /llms/vaardigheden`.

**Query parameters**
| Name | Values |
| --- | --- |
| `vaardigheid` | `Overzicht creëren`, `Kritisch oordelen`, `Juiste kennis ontwikkelen`, `Kwalitatief product maken`, `Plannen`, `Boodschap delen`, `Samenwerken`, `Flexibel opstellen`, `Pro-actief handelen`, `Reflecteren` |
| `niveau` | `1`, `2`, `3`, `4` |

**Response**
```json
{
  "<vaardigheid>": {
    "description": "string",
    "level_description": {
      "1": { "subtitle": "string | null", "description": "string", "extra_description": "string | null" },
      "2": { "subtitle": "string | null", "description": "string", "extra_description": "string | null" },
      "3": { "subtitle": "string | null", "description": "string", "extra_description": "string | null" },
      "4": { "subtitle": "string | null", "description": "string", "extra_description": "string | null" }
    }
  }
}
```

> `GET /api/v1/vaardigheden` still serves the old, unfiltered shape (`{ "1": { "title": ..., "info": ... } }`) for backwards compatibility, but is deprecated (`Deprecation`/`Sunset`/`Link` headers point here) and will be removed after 2026-09-30.

### `GET /api/v1/beroepsrollen`

Returns all ICT-beroepsrollen per gilde. Markdown equivalent: `GET /llms/beroepsrollen`.

**Query parameters**
| Name | Values |
| --- | --- |
| `gilde` | `AI`, `BE`, `BIT`, `CS`, `CI`, `FE`, `UI/UX`, `TI`, `GD` |

**Response**
```json
{
  "<gilde>": {
    "name": "string",
    "description": "string",
    "examples": "string",
    "primary_layer": "string",
    "secondary_layers": ["string"],
    "roadmap": {
      "level_one": "string",
      "level_two": "string",
      "level_three": "string",
      "resources": [{ "text": "string", "url": "string" }],
      "challenges": ["string"]
    },
    "example_jobs": { "<job title>": "string" }
  }
}
```

### `GET /api/v1/hboi`

Returns all HBO-I beroepstaken. A beroepstaak is the combination of an architectuurlaag and an activiteit. Markdown equivalent: `GET /llms/hboi`.

**Query parameters**
| Name | Values |
| --- | --- |
| `architectuurlaag` | `Gebruikersinteractie`, `Organisatieprocessen`, `Infrastructuur`, `Software`, `Hardwareinterfacing` |
| `activiteit` | `Analyseren`, `Adviseren`, `Ontwerpen`, `Realiseren`, `Manage & Control` |
| `niveau` | `1`, `2`, `3`, `4` |

**Response**
```json
{
  "<architectuurlaag> <activiteit>": {
    "1": { "subtitle": "string | null", "description": "string", "extra_description": "string | null" },
    "2": { "subtitle": "string | null", "description": "string", "extra_description": "string | null" },
    "3": { "subtitle": "string | null", "description": "string", "extra_description": "string | null" },
    "4": { "subtitle": "string | null", "description": "string", "extra_description": "string | null" }
  }
}
```

### `GET /api/v1/beroepsproducten`

Returns all beroepsproducten voorbeelden. Markdown equivalent: `GET /llms/beroepsproducten`.

**Query parameters**
| Name | Values |
| --- | --- |
| `architectuurlaag` | see above |
| `activiteit` | see above |
| `gilde` | see above |

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
