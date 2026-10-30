---
name: readme-docs
description: Use this whenever a change affects how developers set up, configure, run, build, or test the project, or what its tech stack is (README.md).
---

# Keeping the README in sync

`README.md` at the repo root is the first stop for a developer joining QViewNG. It covers the project overview and tech stack, dev environment setup, running and building the app, and tests. Whenever a change affects any of those, update the README in the same piece of work.

**Goal:** a developer can go from a fresh machine to a running app and passing tests by following the README alone. We don't want exhaustive detail, just the steps and commands they need, and enough context to know why.

## When to update it

Update the README when a change does any of these:
- Adds, removes, or renames a dependency or tool the developer has to install (e.g. something `getrustdev.sh` installs, Docker, Diesel CLI).
- Adds, removes, or renames an environment variable or `.env*` file, or changes what one is for.
- Changes how the database is created, migrated, or seeded (e.g. `setup-db-valkey.sh`, migration commands, compose services).
- Changes how the app is started, built, or deployed: `npm run …` scripts, `cargo run/build` targets, `run-backend.sh`, `compose*.yaml`, ports.
- Changes how tests are run: `run-tests.sh`, test commands, test env files.
- Changes the tech stack or architecture described in the overview or **How It Works** (e.g. a new cache, a new framework).

## Rules

1. **The README matches the code.** New thing → add it. Changed thing → update it. Removed thing → remove it. While you're in there, if you notice other steps or commands in the section you touched that are wrong or out of date (a wrong script name, a wrong directory, a skipped step number), fix those too and call them out.
2. **Never stage or commit** the README (or the code change that goes with it) until the user has reviewed it. Leave it as an unstaged change.
3. **Always describe the changes** to the user after updating it (see [Reporting](#reporting)).

## How the README is organized

- `# QViewNG`: overview, links (demo, Jira, wiki), the tech stack list, and how to contribute.
- `## Dev Environment Setup`: numbered steps for a fresh Ubuntu 24.04 machine, in the order a developer does them, then bullet points for running and building.
- `### Tests`: unit tests, then integration tests.
- `# How It Works`: short architecture notes.

Put new content where a developer would hit it in that flow. A new setup step goes at the point in the sequence where it's needed. Renumber the following steps and fix any "steps #x, #y" references to them.

### Style
Match what's there:
- Numbered steps like `1)`, each one action, with a one-line explanation of what it does or why.
- Commands in fenced code blocks, indented under their step. Use `{root}` for the repo root in paths.
- Short notes in plain sentences right after the command they apply to (e.g. "Docker deployments don't need this step: …").
- Keep it brief. Link to a file or script instead of repeating what it does in detail.

## Final check: spelling and grammar
As the last step, after everything else is done, proofread the text you added or changed, plus the rest of any section you touched. Correct every spelling and grammar mistake before handing the work back.
- **Spelling:** fix typos and misspellings (e.g. "Thise" → "This", "datbase" → "database"), doubled words ("the the"), and verb/noun mix-ups like "setup" (noun) vs. "set up" (verb).
- **Grammar:** fix subject/verb agreement, wrong word forms ("a inter" → "an inter"), wrong words ("context" → "contact", "listend" → "listens"), missing words, and run-on sentences. Keep the author's wording and tone; fix mistakes, don't rewrite.
- **Names:** write QView, QViewNG, QuizMachine, QMServer, PostgreSQL, Valkey, Docker, Rust, React and Material UI exactly like that in prose.
- **Leave code alone:** don't "correct" anything inside code blocks or backticks, or file names, commands, env variable names or URLs, unless it's actually wrong (e.g. a script name that doesn't match the real file). Then fix it as a content fix under rule 1, not as a spelling fix.
- List what you corrected in your report to the user.

## Reporting

After every update, tell the user:
- Each part of the README **added**, **updated** (what changed), or **removed**, by section and step number.
- Any other out-of-date or wrong content you fixed along the way, and anything you weren't sure about, so they can check it.
- The spelling and grammar corrections you made.
- A reminder that the changes are unstaged and waiting for their review.
