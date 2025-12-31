
-- Rename the division table to orig_division
ALTER TABLE tournaments RENAME to orig_tournaments;
ALTER TABLE divisions RENAME to orig_divisions;

-- create the new tournaments table 
create table tournaments (
    tid UUID PRIMARY KEY DEFAULT gen_random_uuid(),         -- unique id of this tournament
    organization varchar(32) NOT NULL,
    tname varchar(32) NOT NULL,                             -- name of the tournament (human readable)
    breadcrumb varchar(32) NOT NULL,                        -- short name used for urls (i.e. /t/q2022/dn)
    fromdate date NOT NULL,
    todate date NOT NULL,
    venue varchar(64) NOT NULL, 
    city varchar(64) NOT NULL,
    region varchar(64) NOT NULL,
    country varchar(32) NOT NULL,
    contact varchar(64) NOT NULL,
    contactemail varchar(255) NOT NULL,
    is_public boolean NOT NULL DEFAULT false,
    shortinfo varchar(1024) NOT NULL DEFAULT 'No information is available now.  Check back later.',
    info text NOT NULL DEFAULT 'No information is available now.  Check back later.',
    created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tname,organization)
);

-- create the new divisions table 
create table divisions (
    did UUID UNIQUE PRIMARY KEY DEFAULT gen_random_uuid(),  -- division identifier (unique)
    tid UUID NOT NULL,                                      -- tournament id - which tournament this division is in
    dname varchar(32) NOT NULL,                             -- division name
    breadcrumb varchar(32) NOT NULL,                        -- breadcrumb name (used to create short urls)
    is_public boolean NOT NULL DEFAULT true,                -- controls visibility of division for the average user
    shortinfo varchar(1024) NOT NULL DEFAULT 'No information is available now.  Check back later.',  -- human information
    created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tid,dname)
);

-- Drop the original tables
DROP TABLE orig_divisions CASCADE;
DROP TABLE orig_tournaments CASCADE;
