## API- en LLMS-referentie

LEF publiceert vier datasets, elk bereikbaar als JSON (`/api/...`) en als Markdown (`/llms/...`). Beide vormen accepteren dezelfde optionele query-parameters om overfetching te voorkomen. Een filter dat niet relevant is voor een endpoint wordt genegeerd in plaats van een fout te geven. Filters die wel van toepassing zijn worden met AND gecombineerd; als je zowel `architectuurlaag` als `activiteit` weglaat krijg je alle beroepstaken.

### Vaardigheden

- `GET /api/v2/vaardigheden` (JSON) · `GET /llms/vaardigheden` (Markdown)
- Query-parameters:
  - `vaardigheid` — één van: `Overzicht creëren`, `Kritisch oordelen`, `Juiste kennis ontwikkelen`, `Kwalitatief product maken`, `Plannen`, `Boodschap delen`, `Samenwerken`, `Flexibel opstellen`, `Pro-actief handelen`, `Reflecteren`
  - `niveau` — `1`, `2`, `3` of `4`
- Voorbeeld: `/llms/vaardigheden?vaardigheid=Plannen&niveau=2`
- Let op: `/api/v1/vaardigheden` blijft de oude, deprecated vorm zonder filters (zie `Deprecation`/`Sunset`-headers); nieuwe integraties gebruiken `/api/v2/vaardigheden`.

### Beroepsrollen

- `GET /api/v1/beroepsrollen` (JSON) · `GET /llms/beroepsrollen` (Markdown)
- Query-parameters:
  - `gilde` — één van: `AI`, `BE`, `BIT`, `CS`, `CI`, `FE`, `UI/UX`, `TI`, `GD`
- Voorbeeld: `/llms/beroepsrollen?gilde=BE`

### Beroepstaken (HBO-i)

- `GET /api/v1/hboi` (JSON) · `GET /llms/hboi` (Markdown)
- Een beroepstaak is de combinatie van een architectuurlaag en een activiteit. `architectuurlaag` en `activiteit` zijn onafhankelijk van elkaar te gebruiken: geef je alleen `architectuurlaag` mee, dan krijg je alle activiteiten binnen die laag.
- Query-parameters:
  - `architectuurlaag` — één van: `Gebruikersinteractie`, `Organisatieprocessen`, `Infrastructuur`, `Software`, `Hardwareinterfacing`
  - `activiteit` — één van: `Analyseren`, `Adviseren`, `Ontwerpen`, `Realiseren`, `Manage & Control`
  - `niveau` — `1`, `2`, `3` of `4`
- Voorbeeld: `/llms/hboi?architectuurlaag=Software&activiteit=Realiseren`
- Voorbeeld (hele laag): `/llms/hboi?architectuurlaag=Software`

### Beroepsproducten

- `GET /api/v1/beroepsproducten` (JSON) · `GET /llms/beroepsproducten` (Markdown)
- Query-parameters:
  - `architectuurlaag` — zie hierboven
  - `activiteit` — zie hierboven
  - `gilde` — zie hierboven
- Voorbeeld: `/llms/beroepsproducten?gilde=FE&architectuurlaag=Software`
