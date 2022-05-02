// @generated automatically by Diesel CLI.

diesel::table! {
    devices (id) {
        id -> Uuid,
        location -> Varchar,
        token -> Bpchar,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    measurements (id) {
        id -> Uuid,
        humidity -> Float8,
        temperature -> Float8,
        carbon_dioxide -> Float8,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
        location -> Varchar,
        created_by -> Uuid,
    }
}

diesel::joinable!(measurements -> devices (created_by));

diesel::allow_tables_to_appear_in_same_query!(
    devices,
    measurements,
);
