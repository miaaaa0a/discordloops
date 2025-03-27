use std::{collections::HashMap, thread, time};
use anyhow::Error;
use config::load_config;
use tray_icon::{menu::{IsMenuItem, Menu, MenuItem}, Icon, TrayIconBuilder, TrayIconEvent};
use discord_sdk;
use tokio;

pub mod proj_info;
pub mod config;
pub mod presence;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = load_config().unwrap();

    if let Ok(event) = TrayIconEvent::receiver().try_recv() {
        println!("{:?}", event);
    }
    let icon = Icon::from_path("./icon.ico", None).unwrap();
    let menu_items: &[&dyn IsMenuItem] = &[&MenuItem::new("Exit", true, None)];
    let tray_menu = Menu::with_items(menu_items).unwrap();
    let _tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("DiscordLoops")
        .with_icon(icon)
        .build()
        .unwrap();

    let client = presence::make_client(discord_sdk::Subscriptions::ACTIVITY, config.app_id).await;
    let mut activity_events = client.wheel.activity();
    tokio::task::spawn(async move {
        while let Ok(ae) = activity_events.0.recv().await {
            tracing::info!(event = ?ae, "received activity event");
        }
    });


    let wait = time::Duration::from_secs(config.update_rate);
    let fl_hwnd = proj_info::get_fl();
    let mut info: HashMap<&str, String>;

    println!("discord rpc started");

    loop {
        info = proj_info::get_info(&fl_hwnd, &config);
        let rp = discord_sdk::activity::ActivityBuilder::default()
            .details(info["plugins"].to_owned())
            .state(info["project"].to_owned());
        client.discord.update_activity(rp).await?;
        thread::sleep(wait);
    }
}