alter table measurements
    alter column created_at type timestamp,
    alter column updated_at type timestamp;

alter table devices
    alter column created_at type timestamp,
    alter column updated_at type timestamp;