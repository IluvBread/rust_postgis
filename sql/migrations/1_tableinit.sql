CREATE TABLE IF NOT EXISTS world(
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(), 
    location geometry,
    name text);
