use notify_rust::{Notification, Hint}; /* For Notifications */
use super::*;

pub fn run_notify() -> Result<(), Box<dyn Error>> {
    let file: Vec<String> = open_file();

    let mut output: String = String::new();
    for c in file {
        if is_crossedout(&c) == false {
            output += "\n";
            output += &c;
        }
    }

    Notification::new()
        .summary("Rusty Reminder")
        .body(&*output)
        .icon("thunderbird")
        .appname("thunderbird")
        .hint(Hint::Category("TODO: reminder".to_owned()))
        .hint(Hint::Resident(true)) // this is not supported by all implementations
        .timeout(0) // this however is
        .show()?;


    Ok(())
}
