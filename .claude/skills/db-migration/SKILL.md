---
name: db-migration
description: Use whenever a database migration is created.
---

Every migration must preserve the integrity of existing data. Migrations in this project are Diesel migrations in `backend/migrations/` (each has an `up.sql` and a `down.sql`), and they run against databases that already hold real data.

## Before writing any migration code

Work out what happens to every existing row the migration touches:

1. **Identify affected data.** List the tables, columns, constraints, and foreign keys the migration changes, and what existing data lives in them.
2. **Plan how existing data is carried forward.** For each change, decide exactly how current rows end up in the new shape. For example:
   - New NOT NULL column: backfill it (default value or derived from existing data) before adding the constraint.
   - Column rename or move: copy the data across, don't drop and re-add.
   - Type change: confirm every existing value converts cleanly (no truncation, rounding, overflow, timezone shifts, or failed casts).
   - New unique, check, or FK constraint: confirm existing rows already satisfy it, or plan how to fix the ones that don't.
   - Splitting or merging tables or columns: make sure every value has a destination and relationships stay intact.
3. **Check reversibility.** `down.sql` should restore the previous schema *and* its data wherever possible. Note any case where it can't.

## When to stop and notify the user

**Do not write or change any code** if any of these are true:

- You can't find a way to carry existing data forward with its integrity intact.
- The transformation is lossy: data would be dropped, truncated, narrowed, merged ambiguously, or otherwise unrecoverable.
- Existing rows might violate a new constraint and there's no clear, safe way to fix them.
- `down.sql` can't restore the data that `up.sql` changes.

Instead, tell the user:
- What the roadblock is (which tables/columns, and which change causes it).
- What data would be lost or at risk, and roughly how much if you can tell.
- Possible ways forward (e.g. keep the old column for now, archive the data to a side table, do a multi-step migration, or accept the loss), with your recommendation.

Then wait for their decision before going on.

## When writing the migration

- Do backfills and data transformations in the migration itself, in the right order (add nullable → backfill → add constraint).
- Don't drop columns or tables that still hold data unless the user has explicitly approved it.
- When creating a new table, also follow the `sql-table-creation` skill.
