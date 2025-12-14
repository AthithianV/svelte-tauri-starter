use crate::entity::connection_entity;
use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReadConnectionModel {
    pub id: i64,
    pub name: String,
    pub host: String,
    pub port: i32,
    pub username: Option<String>,
    pub password: Option<String>, // ⚠️ encrypt this with stronghold/keychain
    pub db: i32,
    pub last_synced_at: Option<DateTimeUtc>,
    pub bullmq_prefix: String,
    pub is_default: bool,
    pub created_at: DateTimeUtc,
}

impl From<connection_entity::Model> for ReadConnectionModel {
    fn from(entity: connection_entity::Model) -> Self {
        ReadConnectionModel {
            id: entity.id,
            name: entity.name,
            host: entity.host,
            port: entity.port,
            username: entity.username,
            password: entity.password,
            db: entity.db,
            is_default: entity.is_default,
            created_at: entity.created_at,
            last_synced_at: entity.last_synced_at,
            bullmq_prefix: entity.bullmq_prefix,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateConnectionModel {
    pub name: String,
    pub host: String,
    pub port: i32,
    pub username: Option<String>,
    pub password: Option<String>, // ⚠️ encrypt this with stronghold/keychain
    pub db: Option<i32>,
    pub is_default: Option<bool>,
    pub bullmq_prefix: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateConnectionModel {
    pub name: Option<String>,
    pub host: Option<String>,
    pub port: Option<i32>,
    pub username: Option<String>,
    pub password: Option<String>, // ⚠️ encrypt this with stronghold/keychain
    pub db: Option<i32>,
    pub is_default: Option<bool>,
    pub bullmq_prefix: Option<String>,
}
