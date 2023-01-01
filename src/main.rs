use std::process::exit;
use std::io::{stdin, stdout, Write};
use std::env;


fn verbose(input: &str) {
    println!("[*] {}", input);
}
fn error(input: &str, exit_code: i32) {
    println!("[-] ERROR: {}", input);
    /*https://iq.opengenus.org/terminate-and-pause-in-rust/#2usingabortfunction*/
    exit(exit_code);
}



/*  vvv  */



fn shell() {
    /* https://www.joshmcguigan.com/blog/build-your-own-shell-rust/ */
    loop {
        print!(" >>> ");
        /* https://stackoverflow.com/questions/40392906/no-method-named-flush-found-for-type-stdiostdout-in-the-current-scope */
        stdout().flush().expect("Unable to flush stdout");

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut command = input.trim().split_whitespace();

        match command.next().unwrap() {
            "quit" => { break; },
            &_ => continue
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    shell();


    print!("{:?}\n", args);
    verbose("Some thing");
    error("Up o", 1);
}
