ALTER TABLE measurements
    ADD COLUMN location varchar NOT NULL DEFAULT 'unknown',
    ADD COLUMN created_by uuid NULL REFERENCES devices (id);

DO $$
DECLARE
    device_id uuid;
BEGIN
    INSERT INTO devices (location, token, created_at)
        VALUES ('unknown', md5(random()::text), now())
    RETURNING
        id INTO device_id;
    UPDATE
        measurements
    SET
        created_by = device_id
    WHERE
        created_by IS NULL;
END;
$$
LANGUAGE plpgsql;

ALTER TABLE measurements
    ALTER COLUMN created_by SET NOT NULL;

