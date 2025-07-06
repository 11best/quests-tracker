use anyhow::Result;
use axum::async_trait;
use std::sync::Arc;

use crate::{
    domain::{
        entities::guild_commanders::{GuildCommanderEntity, RegisterGuildCommanderEntity},
        repositories::guild_commanders::GuildCommandersRepository,
    },
    infrastructure::postgres::postgres_connection::PgPoolSquad,
};

pub struct GuildCommandersPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl GuildCommandersPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl GuildCommandersRepository for GuildCommandersPostgres {
    async fn register(
        &self,
        register_guild_commander_entity: RegisterGuildCommanderEntity,
    ) -> Result<i32> {
        unimplemented!()
    }

    async fn find_by_username(&self, username: String) -> Result<GuildCommanderEntity> {
        unimplemented!()
    }
}
