use sea_orm::entity::prelude::*;
use super::_entities::posts::{ActiveModel, Entity};
pub type Posts = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
