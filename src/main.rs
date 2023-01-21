#[cfg(test)]
mod test;

const WRITEDIR: &str = "/tmp/foo";
const VERBOSE: bool = true;

/* misc {{{ */
fn is_verbose(input: &str, verbose: bool) {
    if verbose {
        println!("[*] {}", input);
    }
}
fn create_dir(input: &str) -> Result<String, std::io::Error> {
    let s = String::new();
    std::fs::create_dir_all(input).expect("create_dir failed to make file");

    Ok(s)
}
fn get_input() -> String {
    let mut input_text: String = String::new();
    std::io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    input_text
}
/* }}} */
fn shellopen(input: Vec<Vec<String>>) {}





/* vvv */

fn shell(verbose: bool) {
    is_verbose("Starting In Shell Mode", verbose);
    _=create_dir(WRITEDIR);

    let list: Vec<Vec<String>> = Default::default();

    'main: loop {
        let input = get_input();

        match input.split_whitespace().next() {
            Some("o") | Some("open") => { shellopen(list); },
            _ => {},
        }

        break 'main;
    }
}

fn main() {
    shell(VERBOSE);
}
