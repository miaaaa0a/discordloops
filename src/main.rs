#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Error;
use config::load_config;
use std::{collections::HashMap, thread, time};

pub mod config;
pub mod presence;
pub mod proj_info;
pub mod tray_icon;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = load_config().unwrap();

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

    let tray_hwnd = tray_icon::create_window();
    /*if tray_icon::draw_tray_icon(tray_hwnd?)? != true {
        panic!("error while creating tray icon");
    };*/
    //println!("hwnd in main loop: {:?}", tray_hwnd.as_ref().unwrap().hwnd);
    println!("{:?}", tray_icon::draw_tray_icon(tray_hwnd.unwrap().hwnd)?);

    //println!("discord rpc started");

    loop {
        info = proj_info::get_info(&fl_hwnd, &config);
        let rp = discord_sdk::activity::ActivityBuilder::default()
            .details(info["plugins"].to_owned())
            .state(info["project"].to_owned());
        client.discord.update_activity(rp).await?;
        thread::sleep(wait);
    }
}
