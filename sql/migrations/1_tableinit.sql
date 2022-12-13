CREATE TABLE IF NOT EXISTS world(
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(), 
    location geometry,
    name text);

CREATE TABLE public.timeslots (
	id uuid PRIMARY KEY DEFAULT gen_random_uuid(), 
	location_id uuid references world(id),
    start_at TIMESTAMP WITH TIME ZONE NOT NULL,
    end_at TIMESTAMP WITH TIME ZONE NOT NULL
);