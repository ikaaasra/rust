use crate::model::{ToDoModel, UserModel};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct JWT {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct Signup {
    pub name: String,
    pub mail: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct Signin {
    pub mail: String,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct UserSingleResponse {
    pub status: String,
    pub data: UserModel,
}

#[derive(Serialize, Debug)]
pub struct UserListResponse {
    pub status: String,
    pub results: usize,
    pub data: Vec<UserModel>,
}
