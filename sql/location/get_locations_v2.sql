SELECT name, ST_X(location::geometry) AS longitude, ST_Y(location::geometry) AS latitude, vehid, FROM public.world
where public.world.location && ST_MakeEnvelope($1, $2, $3, $4, 4326);

