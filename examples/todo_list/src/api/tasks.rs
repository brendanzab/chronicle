#![allow(unused_variables)]

use rocket_contrib::JSON;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateTaskData {
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChangeDescriptionData {
    pub description: String,
}

#[post("/tasks", data = "<data>")]
pub fn open_account(data: JSON<CreateTaskData>) {
    unimplemented!()
}

#[post("/tasks/<id>/change_description", data = "<data>")]
pub fn deposit_money(id: &str, data: JSON<ChangeDescriptionData>) {
    unimplemented!()
}

#[post("/tasks/<id>/complete")]
pub fn complete(id: &str) {
    unimplemented!()
}

#[post("/tasks/<id>/archive")]
pub fn archive(id: &str) {
    unimplemented!()
}
