use std::{thread, time};
use discord_presence::{Client, Event};
pub mod proj_info;

fn main() {
    let mut drpc = Client::new(1168141266517766175);
    let wait = time::Duration::from_secs(10);
    let mut proj: String;
    let fl_hwnd = proj_info::get_fl();
    let mut otts: String;

    drpc.start();
    let _ready = drpc.block_until_event(Event::Ready);

    println!("discord rpc started");

    loop {
        proj = proj_info::get_project(&fl_hwnd);
        otts = proj_info::count_ott(&fl_hwnd);
        drpc.set_activity(|a| {
            a.state(otts)
            .details(proj)
        }).expect("Failed to set activity");
        thread::sleep(wait);
    }
}