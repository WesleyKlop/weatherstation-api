CREATE TABLE measurements (
    id uuid NOT NULL DEFAULT uuid_generate_v4 () PRIMARY KEY,
    humidity float NOT NULL,
    temperature float NOT NULL,
    carbon_dioxide float NOT NULL,
    created_at timestamp NOT NULL DEFAULT now(),
    updated_at timestamp NULL
);

