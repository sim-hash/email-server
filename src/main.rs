use core::time;

use anyhow::{Context, Result};
use email_server::database;

#[tokio::main]
async fn main() {

//    let db = match database::ClientDatabase::new().await {
//        Ok(db) => db,
//        Err(e) => {
//            println!("Yolo this is me");
//            tracing::error!("Failed to connect to database: {}", e);
//            return;
//        }
//    };

    periodically_clean_db(tokio::time::Duration::from_secs(60));
    std::thread::sleep(time::Duration::from_secs(120));
}

fn periodically_clean_db(period: tokio::time::Duration) {
    std::thread::spawn(move || -> Result<()> {
        tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .enable_io()
            .build()
            .context("failed to build async runtime")?
            .block_on(async move {
                let local = tokio::task::LocalSet::new();
                local.spawn_local(async move {
                    let db = match database::ClientDatabase::new().await {
                        Ok(db) => db,
                        Err(e) => {
                            tracing::error!("Failed to connect to database: {}", e);
                            return;
                        }
                    };
                    let mut interval = tokio::time::interval(period);
                    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
                    loop {
                        interval.tick().await;

//                        if let Err(e) = db.delete_old_mail().await {
//                            tracing::error!("Failed to delete old mail: {}", e);
//                        }
                    }
                });
                local.await;
            });
        Ok(())
    });
}
