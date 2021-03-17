alter table measurements
    add column location   varchar not null default 'unknown',
    add column created_by uuid    null references devices (id);

do
$$
    declare
        device_id uuid;
    begin
        insert into devices (location, token, created_at)
        values ('unknown', md5(random()::text), now())
        returning id into device_id;

        update measurements set created_by = device_id where created_by is null;
    end;
$$ language plpgsql;

alter table measurements
    alter column created_by set not null;