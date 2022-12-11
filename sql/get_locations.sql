SELECT name, ST_X(location::geometry) AS longitude, ST_Y(location::geometry) AS latitude FROM public.world
where public.world.location && ST_MakeEnvelope($1, $2, $3, $4, 4326);

