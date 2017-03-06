table! {
    events(source_id, source_sequence_number) {
        global_sequence_number -> BigInt,
        source_id -> Uuid,
        source_sequence_number -> BigInt,
        data -> Binary,
    }
}
