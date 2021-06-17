select date_trunc('hour', created_at)                        as moment,
       round(avg(humidity)::numeric, 2)::double precision    as humidity,
       min(humidity)                                         as min_humidity,
       max(humidity)                                         as max_humidity,
       round(avg(temperature)::numeric, 2)::double precision as temperature,
       min(temperature)                                      as min_temperature,
       max(temperature)                                      as max_temperature
from measurements
group by moment
order by moment desc;