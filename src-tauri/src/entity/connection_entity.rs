use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue::Set};

/// Entity generated from migration: `connection`
///
/// Migration fields:
/// - id (integer, auto-increment, primary key)
/// - name (text, not null)
/// - host (text, not null)
/// - port (integer, not null)
/// - password (text, null)
/// - db (integer, default 0)
/// - is_default (boolean, default false)
/// - created_at (timestamp, not null, DEFAULT CURRENT_TIMESTAMP)
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "connection")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub host: String,
    pub port: i32,
    pub password: Option<String>,
    pub username: Option<String>,
    pub db: i32,
    pub is_default: bool,
    pub last_synced_at: Option<DateTimeUtc>,
    pub bullmq_prefix: String,
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::tab_entity::Entity")]
    Tab,
    #[sea_orm(has_many = "super::folder_entity::Entity")]
    Folder,
    #[sea_orm(has_many = "super::queue_entity::Entity")]
    Queue,
}

impl Related<super::tab_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tab.def()
    }
}
impl Related<super::folder_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Folder.def()
    }
}
impl Related<super::queue_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Queue.def()
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    /// Ensure `created_at` is set on insert. The migration only defines `created_at`
    /// (no explicit `updated_at`), so we only set `created_at` here.
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            let now = Utc::now();
            self.created_at = Set(now);
        }

        Ok(self)
    }
}
