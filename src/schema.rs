table! {
    measurements (uuid) {
        uuid -> Binary,
        id -> Char,
        humidity -> Float,
        temperature -> Float,
        carbon_dioxide -> Float,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}
