CREATE TABLE devices (
    id uuid NOT NULL DEFAULT uuid_generate_v4 () PRIMARY KEY,
    location varchar NOT NULL,
    token char(32) NOT NULL,
    created_at timestamp NOT NULL DEFAULT now(),
    updated_at timestamp NULL
);

