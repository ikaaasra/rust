use crate::model::ToDo;
use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct ToDoData {
    pub todo: Todo,
}

#[derive(Serialize, Debug)]
pub struct ToDoSingleResponse {
    pub status: String,
    pub data: ToDo,
}

#[derive(Serialize, Debug)]
pub struct ToDoListResponse {
    pub status: String,
    pub results: usize,
    pub data: Vec<ToDo>,
}
