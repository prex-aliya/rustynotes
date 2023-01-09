use std::io::{stdin, stdout, Write, BufReader, BufRead};
use notify_rust::Notification; /* For Notifications */
use chrono::{DateTime, Utc}; /* For Local Date */
use std::time::Duration; /* Convert from seconds */
use std::process::exit; /* To Exit On Failure */
use std::thread::sleep; /* Notifications Wait Between others */
use std::error::Error;
use std::path::Path;
use std::env;

const PROMPT: &str = ">>>";
const WRITEDIR: &str = "/tmp/foo";



/* Misc {{{ */
fn _help() {
}
fn verbose(input: &str, print: bool) {
    if print == true {
        println!("[*] {}", input);
    }
}
fn _warning() {
}
fn exit_error(input: &str, exit_code: i32) {
    println!("[-] ERROR: {}", input);
    /*https://iq.opengenus.org/terminate-and-pause-in-rust/#2usingabortfunction*/
    exit(exit_code);
}

fn get_date() -> String {
    let now: DateTime<Utc> = Utc::now();

    now.format("%d-%m-%Y\n").to_string()
}
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    if !std::path::Path::new(WRITEDIR).exists() {
        std::fs::File::create(WRITEDIR).ok();
    }
    let file = std::fs::File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
fn write_to_file(input: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut file = std::fs::File::create(WRITEDIR)?;
    writeln!(file, "{}", input.join("\n").trim())?;
    Ok(())
}
fn is_crossedout(input: &String) -> bool {
    let mut crossnum: i8 = 0;
    for c in input.chars() {
        if c == '~' {
            crossnum += 1;
        }
    }
    if crossnum >= 4 {
        return true;
    }

    false
}
fn is_string_numeric(str: &String) -> bool {
    for c in str.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}
/* }}} */
/* Shell {{{  */
fn number_input(string: String) -> Vec<String> {
    let mut secondary: Vec<String> = Vec::new();
    let command = (string.trim().split_whitespace()).collect::<Vec<&str>>();

    if command.len() >= 1 {
        for (num, x) in command.iter().enumerate() {
            secondary.push("".to_string());
            for c in x.chars() {
                if c.is_numeric() {
                    secondary[num].push(c);
                }
            }
        }
    }
    secondary.remove(0);

    secondary
}
fn plist(input: &Vec<String>, date: &String) {
    println!("[  \x1b[0;34m{}\x1b[0m  ]", date.trim());

    for (num, note) in input.iter().enumerate() {
        if num > 0 {
            println!("\t[{}] {}", num, note.trim());
        }
    }
}
fn get_input(prompt: &str) -> String {
        print!(" {} ", prompt);

        stdout().flush().expect("Unable to flush stdout");

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input
}
fn premove(string: String, list: &mut Vec<String>) -> &mut Vec<String> {
    let secondary: Vec<String> = number_input(string);

    for x in (0..secondary.len()).rev() {
        if secondary[x].parse::<usize>().is_ok() {
            let num: usize = secondary[x].parse().unwrap();
            if num <= list.len()-1 {
                if !num > 1 {
                    list.remove(num);
                }
            }
        }
    }

    list
}
fn open_file() -> Vec<String> {
    let output: Vec<String> = lines_from_file(WRITEDIR);
    output
}
fn cross_off_list(list: &mut Vec<String>, string: String) -> &mut Vec<String> {
    println!("{}", string);
    let secondary: Vec<String> = number_input(string);

    for x in (0..secondary.len()).rev() {
        if is_string_numeric(&secondary[x]) {
            let num: usize = secondary[x].parse().unwrap();
            if num < list.len() {
                if !num > 1 {
                    if list[num].contains("\x1b[0;31m~~") {
                        list[num] = list[num].replace("\x1b[0;31m~~", "");
                        list[num] = list[num].replace("~~\x1b[0m", "");
                    } else {
                        list[num] = format!("\x1b[0;31m~~{}~~\x1b[0m", list[num].trim());
                    }
                }
            }
        }
    }

    list
}
fn command_add(input: &mut String, command: &str) -> String {
    if command.len() > 1 {
        for _x in 0..=command.len() {
            input.remove(0);
        }
    } else {
        exit_error("Less Than 1 Char in Length of Command", 1);
    }
    input.to_string()
}
/* }}} */
/* Notify {{{ */
fn run_notify() -> Result<(), Box<dyn Error>> {
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
        .show()?;

    Ok(())
}
/* }}} */



/*  vvv  */



fn shell(is_verbose: bool) {
    verbose("Starting in Shell Mode", is_verbose);

    //let date: String = get_date();
    let mut list: Vec<String> = vec![get_date()];
    verbose(&*format!("Current date is: {}", list[0]), is_verbose);


    'main: loop {
        let input = get_input(PROMPT).trim().to_string();

        if !input.is_empty() { /* check if there is an empty input */
            let command = input.trim().split_whitespace().next().unwrap();

            match command {
                "quit" | "exit" => { break 'main; },
                "add"           => { list.push(command_add(&mut input.to_string(), command).to_string()); },
                "ls" | "list"   => { plist(&list, &list[0]); },
                "rm"            => { list = premove(input, &mut list).to_vec(); },
                "w" | "write"   => { write_to_file(&list); },
                "o" | "open"    => { list = open_file(); plist(&list, &list[0]); },
                "c" | "cross"   => { list = cross_off_list(&mut list, input).to_vec();
                                    plist(&list, &list[0]); },
                "d" | "date"    => { list[0] = get_date(); },
                &_  => continue
            }
        }
    }
}

fn notify(is_verbose: bool) {

    verbose("Starting Notifications", is_verbose);

    loop {
        run_notify();
        sleep(Duration::from_secs(1800));
    }

}

fn main() {
    print!("\u{0008}");
    let args: Vec<String> = env::args().collect();

    let is_verbose: bool = true;

    for arg in args {
        match &*arg {
            "-s" | "--shell"    => { shell(is_verbose); }
            "-n" | "--notify"   => { notify(is_verbose); exit(0); },
            &_ => { continue },
        }
    }

    verbose("Defaulting to Default Operation", is_verbose);
    shell(is_verbose);
}
