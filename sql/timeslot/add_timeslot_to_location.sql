INSERT INTO public.timeslots(location_id, start_at, end_at)
values ($1,$2,$3)
    RETURNING id;