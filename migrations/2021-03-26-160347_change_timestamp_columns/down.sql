ALTER TABLE measurements
    ALTER COLUMN created_at TYPE timestamp,
    ALTER COLUMN updated_at TYPE timestamp;

ALTER TABLE devices
    ALTER COLUMN created_at TYPE timestamp,
    ALTER COLUMN updated_at TYPE timestamp;

