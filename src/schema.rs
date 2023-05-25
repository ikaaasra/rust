use crate::model::ToDoModel;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateToDo {
    pub title: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateToDo {
    pub title: Option<String>,
    pub content: Option<String>,
    pub complete: Option<bool>,
}

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct ToDoData {
    pub todo: ToDoModel,
}

#[derive(Serialize, Debug)]
pub struct ToDoSingleResponse {
    pub status: String,
    pub data: ToDoModel,
}

#[derive(Serialize, Debug)]
pub struct ToDoListResponse {
    pub status: String,
    pub results: usize,
    pub data: Vec<ToDoModel>,
}
