use diesel::data_types::PgTimestamp;
use uuid::Uuid;

use schema::events;


#[derive(Debug, Clone, Copy, Insertable)]
#[table_name="events"]
pub struct NewEvent<'a> {
    pub source_id: Uuid,
    pub sequence_number: i64,
    pub payload: &'a [u8],
}


#[derive(Debug, Clone, Queryable)]
pub struct Event {
    pub offset: i64,
    pub source_id: Uuid,
    pub sequence_number: i64,
    pub payload: Vec<u8>,
    pub created_at: PgTimestamp,
}
