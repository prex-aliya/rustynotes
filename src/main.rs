use std::fs;

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
    fs::create_dir_all(input).expect("create_dir failed to make file");

    Ok(s)
}
/* }}} */

/* vvv */

fn shell(verbose: bool) {
    is_verbose("", verbose);


}

fn main() {
    _=create_dir(WRITEDIR);
    shell(VERBOSE);
}
