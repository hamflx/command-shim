use std::process::Command;

use libsui::find_section;

fn main() {
    let command_data = find_section("command").unwrap();
    let command = String::from_utf8(command_data.into()).unwrap();
    if let Some(code) = Command::new(command)
        .args(std::env::args().skip(1))
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .code()
    {
        std::process::exit(code);
    }
}
