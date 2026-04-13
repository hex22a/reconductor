CREATE SCHEMA IF NOT EXISTS recon;

CREATE TABLE recon.schema_migrations (
    version TEXT PRIMARY KEY,
    applied_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE recon.users (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    password_version SMALLINT NOT NULL DEFAULT 1,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now(),
    last_login_at TIMESTAMP NOT NULL DEFAULT now(),
    is_active BOOLEAN NOT NULL DEFAULT true
);

CREATE TABLE recon.projects (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    owner_id UUID NOT NULL REFERENCES recon.users (id),
    name TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE INDEX idx_projects_id_desc ON recon.projects (id DESC);

CREATE TYPE scan_status as ENUM ('scheduled', 'in progress', 'done');

CREATE TABLE recon.scans (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    project_id UUID NOT NULL REFERENCES recon.projects (id),
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    target INET NOT NULL,
    status scan_status NOT NULL DEFAULT 'scheduled',
    schedule TEXT DEFAULT NULL,
    next_run_at TIMESTAMPTZ
);

CREATE INDEX idx_scans_id_desc ON recon.scans (id DESC);

CREATE TABLE recon.scan_hosts (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    scan_id UUID NOT NULL REFERENCES recon.scans(id),
    ip INET NOT NULL,
    mac MACADDR,
    vendor TEXT,
    hostname TEXT,
    os_match TEXT,
    os_accuracy INT,
    created_at TIMESTAMP DEFAULT now()
);

CREATE INDEX idx_scan_hosts_id_desc ON recon.scan_hosts (id DESC);

CREATE TABLE recon.scan_ports (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    host_id UUID NOT NULL REFERENCES recon.scan_hosts(id),
    port INT NOT NULL,
    protocol VARCHAR(10),
    state VARCHAR(20),
    service TEXT,
    product TEXT,
    version TEXT
);

CREATE INDEX idx_scan_ports_id_desc ON recon.scan_ports (id DESC);
