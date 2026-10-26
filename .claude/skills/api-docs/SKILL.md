---
name: api-docs
description: Use this whenever an endpoint in dir backend/src/services is created or modified.
---

# Keeping the API documentation in sync

`docs/QView API Documentation.docx` is the developer-facing overview of every endpoint defined in `backend/src/services/`. Whenever an endpoint there is added, changed, or removed, update the document in the same piece of work.

**Goal:** every endpoint is documented. We don't want exhaustive detail, just enough for a developer new to the codebase to quickly find what exists for getting each type of data and be effective.

## Rules

1. **Every endpoint is documented.** New endpoint → add it. Changed path, method, parameters, body, or return → update its entry. Removed endpoint → remove its entry. While you're in there, if you notice other endpoints in the touched service file that are missing or out of date, fix those too and call them out.
2. **Bump the version and date on every update.** The title is the first paragraph: `QView API Documentation (v180_2026.09.23)`. Increment the number by 1 and set the date to today in `YYYY.MM.DD` format, e.g. `v181_2026.10.26`. Do this once per update, not once per endpoint.
3. **Never stage or commit** the document (or the code change that goes with it) until the user has reviewed it. Leave it as an unstaged change.
4. **Always describe the changes** to the user after updating it (see [Reporting](#reporting)).

## How the document is organized

- Title (version + date), then `base_uri = http://localhost:3000`, then a note explaining the ordering, then a bold, larger heading **Domain Entities Endpoints**.
- **Sections are based on the type of data the endpoints return**, not on the URL path or the service file. E.g. `GET Admins of Tournament` lives under **Admins** because it returns Users acting as admins, even though its path is under `/tournaments`. Section headings are bold, unnumbered paragraphs, roughly alphabetical (e.g. **Divisions**, **Equipment**, **Games**, **Teams**, **Users**). Identity-type sections carry a note like `(*one of several identities that a User can take on)`.
- If a new endpoint returns a type with no section yet, add a new section in alphabetical position.

### Endpoint entries
Each endpoint is a numbered-list block. The title is at list level 0, its fields at level 1, and sub-items at levels 2–3:

```
GET Games of Round                                   ← level 0: title
  built_including_tests: true                        ← level 1 ("true"/"false" in bold)
  HTTP method: ‘GET’
  endpoint: {base_uri}/api/rounds/{id}/games
  description: <one line, or blank if the title says it all>
  request body:  / body:                             ← only for POST/PUT
    <struct name, or "only fields that you want to update … need to be provided">
  parameters:          (or "parameters: none")
    id                                               ← level 2: param name
      Required                                       ← level 3, red (FF0000); "Optional" otherwise
      ID of the Round the Games belong to            ← level 3: what it is
  returns:
    an array of Games whose parent is the Round      ← level 2
```

- **`built_including_tests`**: `true` if there's an integration test covering the endpoint under `backend/tests/`, otherwise `false`. Check; don't assume.
- **Endpoint path**: always `{base_uri}/api/...`, with path params in `{braces}` and query params written as `?page={page_num}&page_size={num_of_rows}`.
- **Returns**: name the struct or type, and say whether it's a single item, an array, or "a 200 OK response only".

### Titles
Titles say **what is returned and at what scope**, starting with the HTTP method:
- `GET {Things} of {Parent}`: a list scoped to a parent (e.g. `GET Rooms of Tournament`)
- `GET {Thing} (by ID)`: a single item
- `POST/Create {Thing}`, `PUT/Update {Thing}`, `DELETE {Thing} (by ID)`
- Actions use `POST/{Verb phrase}`, e.g. `POST/Add Game to StatsGroup`, `POST/Make User an Admin of a Tournament`.

Use your best judgement on the clearest name when an endpoint doesn't fit these patterns.

### Order within a section
Use your best judgement, but follow the existing pattern: GETs first (broadest scope → narrowest, then by-ID), then POST/Create, then other POST actions, then PUT/Update, then DELETE.

## Editing the .docx

All paragraphs use the `normal1` style. Structure comes from direct formatting and the shared numbered list (`numId` 1, `ilvl` 0–3), so edit the XML rather than regenerating the document:

1. Unzip the `.docx` into a fresh folder in the scratchpad.
2. In `word/document.xml`, copy the paragraphs of a similar existing endpoint as the template (same numbering, indents, bold and red runs), and change the text. Escape `&` as `&amp;`. Keep the curly quotes (‘GET’) the document already uses.
3. Re-zip in place over the original, keeping all other parts unchanged.
4. Verify: `soffice --headless --convert-to txt` the result into the scratchpad and check that the new or changed entries read correctly and that the file still opens.

## Reporting

After every update, tell the user:
- The new version string, e.g. `v180_2026.09.23 → v181_2026.10.26`.
- Each endpoint **added**, **updated** (what changed), or **removed**, grouped by section, with its title and method/path.
- Any new sections, and anything you guessed at (naming, placement, `built_including_tests` value) so they can check it.
- A reminder that the changes are unstaged and waiting for their review.
