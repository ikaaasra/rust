use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[allow(non_snake_case)]
#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct ToDoModel {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub complete: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, FromRow, Serialize, Clone)]
pub struct UserModel {
    pub id: Uuid,
    pub name: String,
    pub mail: String,
    pub password: String,
    pub role: String,
    pub photo: String,
    pub verify: bool,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}
