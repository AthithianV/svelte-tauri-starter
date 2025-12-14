pub mod connection_entity;
pub mod folder_entity;
pub mod folder_queue_entity;
pub mod queue_entity;
pub mod settings_entity;
pub mod tab_entity;

pub use connection_entity::{
    ActiveModel as ConnectionActiveModel, Column as ConnectionColumn, Entity as ConnectionEntity,
    Model as ConnectionModel,
};

pub use queue_entity::{
    ActiveModel as QueueActiveModel, Column as QueueColumn, Entity as QueueEntity,
    Model as QueueModel,
};

pub use tab_entity::{
    ActiveModel as TabActiveModel, Column as TabColumn, Entity as TabEntity, Model as TabModel,
};
