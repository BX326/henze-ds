//! Discord bot components: event handling and message formatting.

pub mod handler;
pub mod messages;

pub use handler::{DbPoolKey, Handler};
pub use messages::{announce_bet_placed, announce_bet_result, send_daily_bets};
