use crate::entity::{ConnectionActiveModel, ConnectionEntity};
use crate::model::connection_model::{
    CreateConnectionModel, ReadConnectionModel, UpdateConnectionModel,
};
use anyhow::{bail, Context, Result};
use sea_orm::{prelude::*, ActiveModelTrait, DeleteResult, EntityTrait, Set};

pub struct ConnectionRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> ConnectionRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    // --- CREATE ---
    pub async fn create(&self, data: CreateConnectionModel) -> Result<ReadConnectionModel> {
        let bullmq_prefix = data.bullmq_prefix.unwrap_or("bull".to_string());
        let new_connection = ConnectionActiveModel {
            id: Default::default(), // Auto-incremented
            name: Set(data.name),
            host: Set(data.host),
            port: Set(data.port),
            password: Set(data.password),
            username: Set(data.username),
            db: Set(data.db.unwrap_or(0)),
            is_default: Set(data.is_default.unwrap_or(false)),
            bullmq_prefix: Set(bullmq_prefix),
            created_at: Default::default(),
            last_synced_at: Set(None),
        };

        let inserted_row = new_connection
            .insert(self.db)
            .await
            .with_context(|| format!("Failed to create new Connection"))?;

        let result = ReadConnectionModel::from(inserted_row);
        Ok(result)
    }

    // --- GET ALL ---
    pub async fn get_all(&self) -> Result<Vec<ReadConnectionModel>> {
        let connection_model_vec = ConnectionEntity::find()
            .all(self.db)
            .await
            .with_context(|| format!("Failed to get all Connections"))?;

        Ok(connection_model_vec
            .into_iter()
            .map(ReadConnectionModel::from)
            .collect())
    }

    // --- GET BY ID ---
    pub async fn get_by_id(&self, id: i64) -> Result<Option<ReadConnectionModel>> {
        let connection_model_opt = ConnectionEntity::find_by_id(id)
            .one(self.db)
            .await
            .with_context(|| format!("Failed to get connection for {}", id))?;

        Ok(connection_model_opt.map(ReadConnectionModel::from))
    }

    // --- UPDATE ---
    pub async fn update(
        &self,
        id: i64,
        data: UpdateConnectionModel,
    ) -> Result<ReadConnectionModel> {
        // 1. Find existing record (Must be converted to ActiveModel to update)
        let conn_model = ConnectionEntity::find_by_id(id)
            .one(self.db)
            .await
            .with_context(|| format!("Failed to find connection with ID: {} for Update", id))?
            .ok_or_else(|| anyhow::anyhow!("Connection with ID {} not found", id))?; // Use anyhow to bail

        // 2. Convert to ActiveModel
        let mut active: ConnectionActiveModel = conn_model.into();

        // 3. Update only the fields that are Some(...)
        if let Some(name) = data.name {
            active.name = Set(name);
        }
        if let Some(host) = data.host {
            active.host = Set(host);
        }
        if let Some(port) = data.port {
            active.port = Set(port);
        }
        if let Some(username) = data.username {
            active.username = Set(Some(username));
        }
        if let Some(password) = data.password {
            active.password = Set(Some(password));
        }
        if let Some(db) = data.db {
            active.db = Set(db);
        }
        if let Some(is_default) = data.is_default {
            active.is_default = Set(is_default);
        }
        if let Some(bullmq_prefix) = data.bullmq_prefix {
            active.bullmq_prefix = Set(bullmq_prefix);
        }

        // 4. Save updates. ActiveModel only updates Set fields.
        let updated = active
            .update(self.db)
            .await
            .context("Failed to save updated connection to database")?;

        Ok(ReadConnectionModel::from(updated))
    }

    // --- DELETE ---
    pub async fn delete(&self, id: i64) -> Result<DeleteResult> {
        let res = ConnectionEntity::delete_by_id(id)
            .exec(self.db)
            .await
            .with_context(|| format!("Failed to execute delete for connection ID {}", id))?;

        if res.rows_affected == 0 {
            bail!("Connection with ID {} not found", id);
        }

        Ok(res)
    }
}
