table! {
    devices (id) {
        id -> Uuid,
        location -> Varchar,
        token -> Bpchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    measurements (id) {
        id -> Uuid,
        humidity -> Float8,
        temperature -> Float8,
        carbon_dioxide -> Float8,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        location -> Varchar,
        created_by -> Uuid,
    }
}

joinable!(measurements -> devices (created_by));

allow_tables_to_appear_in_same_query!(devices, measurements,);
