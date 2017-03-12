table! {
    events(source_id, sequence_number) {
        offset -> BigInt,
        source_id -> Uuid,
        sequence_number -> BigInt,
        payload -> Binary,
        created_at -> Timestamp,
    }
}
