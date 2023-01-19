/* Variables {{{ */
const WRITEDIR: &str    = "/tmp/foo";
/* }}}*/



/* misc {{{ */
is_verbose(input: &str, verbose: bool) {
    if verbose {
        println!("[*] {}", input);
    }
}
/* }}} */



/* vvv */



fn shell(verbose: bool) {
    is_verbose("", verbose);
}

fn main() {
    let verbose = true;
    shell(verbose);
}
