use std::{collections::HashMap, thread, time};
use config::load_config;
use discord_presence::{Client, Event};
pub mod proj_info;
pub mod config;

fn main() {
    let config = load_config().unwrap();

    let mut drpc = Client::new(1168141266517766175);
    let wait = time::Duration::from_secs(config.update_rate);
    let fl_hwnd = proj_info::get_fl();
    let mut info: HashMap<&str, String>;

    drpc.start();
    let _ready = drpc.block_until_event(Event::Ready);

    println!("discord rpc started");
    //println!("config: {:?}", config);

    loop {
        info = proj_info::get_info(&fl_hwnd, &config);
        drpc.set_activity(|a| {
            a.state(info["project"].clone())
            .details(info["plugins"].clone())
        }).expect("Failed to set activity");
        thread::sleep(wait);
    }
}