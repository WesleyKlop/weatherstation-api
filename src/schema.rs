table! {
    measurements (uuid) {
        uuid -> Binary,
        id -> Varchar,
        humidity -> Float,
        temperature -> Float,
        carbon_dioxide -> Float,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}
