use uuid::Uuid;

use schema::events;


#[derive(Debug, Clone, Copy, Insertable)]
#[table_name="events"]
pub struct NewEvent<'a> {
    pub source_id: Uuid,
    pub source_sequence_number: i64,
    pub data: &'a [u8],
}


#[derive(Debug, Clone, Queryable)]
pub struct Event {
    pub global_sequence_number: i64,
    pub source_id: Uuid,
    pub source_sequence_number: i64,
    pub data: Vec<u8>,
}
