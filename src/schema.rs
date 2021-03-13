table! {
    measurements (id) {
        id -> Uuid,
        humidity -> Float8,
        temperature -> Float8,
        carbon_dioxide -> Float8,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}
