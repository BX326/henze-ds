//! Settlement checker for pending bets.
//!
//! Polls the Danske Spil API to check if pending bets have been settled
//! and updates the database accordingly.

use henze_ds::{check_outcome_result, BetResult};
use serenity::all::{ChannelId, Http};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;
use tracing::{error, info, warn};

use crate::config::get_announcement_channel_id;
use crate::db::{self, BetStatus, DbPool, RunStatus};
use crate::discord::announce_bet_result;

/// Start the settlement checker background task.
///
/// This runs on an interval, checking all pending bets for results.
pub async fn start_settlement_checker(http: Arc<Http>, db: DbPool, interval_secs: u64) {
    info!(
        "Starting settlement checker with {}s interval",
        interval_secs
    );

    let mut ticker = interval(Duration::from_secs(interval_secs));

    loop {
        ticker.tick().await;
        check_pending_bets(&http, &db).await;
    }
}

/// Check all pending bets for settlement.
async fn check_pending_bets(http: &Http, db: &DbPool) {
    let pending_bets = match db::get_all_pending_bets(db) {
        Ok(bets) => bets,
        Err(e) => {
            error!("Failed to fetch pending bets: {:?}", e);
            return;
        }
    };

    if pending_bets.is_empty() {
        return;
    }

    info!("Checking {} pending bets for settlement", pending_bets.len());

    for (bet, run) in pending_bets {
        match check_outcome_result(&bet.event_id, &bet.outcome_id).await {
            Ok(BetResult::Pending) => {
                // Still waiting, do nothing
            }
            Ok(BetResult::Won) => {
                info!(
                    "Bet {} for run {} WON! Updating...",
                    bet.id, run.id
                );
                handle_bet_won(http, db, &bet, &run).await;
            }
            Ok(BetResult::Lost) => {
                info!(
                    "Bet {} for run {} LOST. Ending run...",
                    bet.id, run.id
                );
                handle_bet_lost(http, db, &bet, &run).await;
            }
            Ok(BetResult::Void) => {
                info!(
                    "Bet {} for run {} was VOIDED. Refunding...",
                    bet.id, run.id
                );
                handle_bet_void(http, db, &bet, &run).await;
            }
            Ok(BetResult::NotFound) => {
                warn!(
                    "Bet {} event/outcome not found in API (event_id={}, outcome_id={})",
                    bet.id, bet.event_id, bet.outcome_id
                );
                // The event may have been removed from the API after conclusion
                // We could mark as void or keep checking - for now, keep checking
            }
            Err(e) => {
                error!(
                    "Error checking bet {} result: {:?}",
                    bet.id, e
                );
            }
        }
    }
}

/// Handle a winning bet.
async fn handle_bet_won(http: &Http, db: &DbPool, bet: &db::Bet, run: &db::BetRun) {
    let new_amount = bet.stake * bet.odds;

    // Update bet status
    if let Err(e) = db::update_bet_status(db, bet.id, BetStatus::Won) {
        error!("Failed to update bet {} status: {:?}", bet.id, e);
        return;
    }

    // Update run amount
    if let Err(e) = db::update_bet_run(db, run.id, RunStatus::Active, new_amount) {
        error!("Failed to update run {} amount: {:?}", run.id, e);
        return;
    }

    // Announce in the configured announcement channel
    if let Some(channel_id) = get_announcement_channel_id() {
        let run_number = db::get_run_number(db, &run.guild_id, run.id)
            .unwrap_or(Some(1))
            .unwrap_or(1);
        
        // Create updated run struct with new amount
        let mut updated_run = run.clone();
        updated_run.current_amount = new_amount;

        if let Err(e) = announce_bet_result(
            http,
            ChannelId::new(channel_id),
            run_number,
            bet,
            &updated_run,
            BetStatus::Won,
        )
        .await
        {
            error!("Failed to announce bet win: {:?}", e);
        }
    }
}

/// Handle a losing bet (ends the run).
async fn handle_bet_lost(http: &Http, db: &DbPool, bet: &db::Bet, run: &db::BetRun) {
    // Update bet status
    if let Err(e) = db::update_bet_status(db, bet.id, BetStatus::Lost) {
        error!("Failed to update bet {} status: {:?}", bet.id, e);
        return;
    }

    // Update run status to lost with 0 amount
    if let Err(e) = db::update_bet_run(db, run.id, RunStatus::Lost, 0.0) {
        error!("Failed to update run {} status: {:?}", run.id, e);
        return;
    }

    // Announce in the configured announcement channel
    if let Some(channel_id) = get_announcement_channel_id() {
        let run_number = db::get_run_number(db, &run.guild_id, run.id)
            .unwrap_or(Some(1))
            .unwrap_or(1);
        
        // Create updated run struct
        let mut updated_run = run.clone();
        updated_run.status = RunStatus::Lost;
        updated_run.current_amount = 0.0;

        if let Err(e) = announce_bet_result(
            http,
            ChannelId::new(channel_id),
            run_number,
            bet,
            &updated_run,
            BetStatus::Lost,
        )
        .await
        {
            error!("Failed to announce bet loss: {:?}", e);
        }
    }
}

/// Handle a voided bet (refund stake).
async fn handle_bet_void(http: &Http, db: &DbPool, bet: &db::Bet, run: &db::BetRun) {
    // Update bet status to void
    if let Err(e) = db::update_bet_status(db, bet.id, BetStatus::Void) {
        error!("Failed to update bet {} status: {:?}", bet.id, e);
        return;
    }

    // Run amount stays the same (stake refunded)
    // No need to update run

    // Announce in the configured announcement channel
    if let Some(channel_id) = get_announcement_channel_id() {
        let run_number = db::get_run_number(db, &run.guild_id, run.id)
            .unwrap_or(Some(1))
            .unwrap_or(1);

        if let Err(e) = announce_bet_result(
            http,
            ChannelId::new(channel_id),
            run_number,
            bet,
            run,
            BetStatus::Void,
        )
        .await
        {
            error!("Failed to announce bet void: {:?}", e);
        }
    }
}
