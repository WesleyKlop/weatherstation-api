ALTER TABLE measurements
    ALTER COLUMN created_at TYPE timestamptz,
    ALTER COLUMN updated_at TYPE timestamptz;

ALTER TABLE devices
    ALTER COLUMN created_at TYPE timestamptz,
    ALTER COLUMN updated_at TYPE timestamptz;

