---
name: sql-table-creation
description: Use this whenever a new table is being created in the database.
---

Whenever a new table is created in the database, make sure to always include the following:

1. 1 column called `created_date` which stores datetime data
2. 1 column called `last_modified_date` which also stores datetime data
3. 1 column for `last_modified_userid` which has data type of UUID as a FK to the Users table (or equivalent)
4. 1 column for `created_by_userid` which has the same data type as `last_modified_userid` (UUID) and is also a FK to the Users table (or equivalent)
5. 1 column for 'del_fl' which is a boolean defaulted to false used for shallow data deletion so that "deleted" data is instead retained in the database while never being displayed to the end user (except for maybe the superuser in some specified contexts).

Make all four of these required (NOT NULL) as the table's audit columns.
