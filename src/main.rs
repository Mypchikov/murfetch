use sys_info::{os_type, os_release};
mod modules;
use modules::uptime::get_uptime;
use modules::uptime::format_uptime;

fn main() {
    let os_type = os_type().unwrap();
    let os_release = os_release().unwrap();

    println!("{} {}", "System:", os_type);
    println!("{} {}", "Kernel:", os_release);
    match get_uptime() {
        Ok(uptime) => {
            println!("{} {}","Uptime:", format_uptime(uptime));
        }
        Err(e) => {
            eprintln!("{} {}","Uptime:", e);
        }
    }
}