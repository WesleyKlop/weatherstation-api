select date_trunc('hour', created_at) as moment,
       avg(humidity)                  as humidity,
       min(humidity)                  as min_humidity,
       max(humidity)                  as max_humidity,
       avg(temperature)               as temperature,
       min(temperature)               as min_temperature,
       max(temperature)               as max_temperature
from measurements
group by moment
order by moment desc
limit
