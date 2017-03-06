#![allow(unused_variables)]

use rocket_contrib::{JSON, UUID};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateTaskData {
    pub description: Uuid,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChangeDescriptionData {
    pub description: Uuid,
}

#[post("/tasks", data = "<data>")]
pub fn open_account(data: JSON<CreateTaskData>) {
    unimplemented!()
}

#[post("/tasks/<id>/change_description", data = "<data>")]
pub fn deposit_money(id: UUID, data: JSON<ChangeDescriptionData>) {
    unimplemented!()
}

#[post("/tasks/<id>/complete")]
pub fn complete(id: UUID) {
    unimplemented!()
}

#[post("/tasks/<id>/archive")]
pub fn archive(id: UUID) {
    unimplemented!()
}
