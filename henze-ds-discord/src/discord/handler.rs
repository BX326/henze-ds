//! Discord event handler implementation.

use serenity::prelude::*;
use tracing::info;

/// Discord event handler for bot lifecycle events.
pub struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: serenity::model::gateway::Ready) {
        info!("{} is connected!", ready.user.name);
    }
}
