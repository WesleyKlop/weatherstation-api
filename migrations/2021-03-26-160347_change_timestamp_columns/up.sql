alter table measurements
    alter column created_at type timestamptz,
    alter column updated_at type timestamptz;

alter table devices
    alter column created_at type timestamptz,
    alter column updated_at type timestamptz;