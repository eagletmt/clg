extern crate clg;
extern crate env_logger;

fn main() {
    env_logger::init();
    let app = clg::cli::build_cli();

    let result = match app.get_matches().subcommand() {
        ("clone", Some(submatch)) => clg::command::clone(submatch),
        ("look", Some(submatch)) => clg::command::look(submatch),
        ("list", Some(submatch)) => clg::command::list(submatch),
        ("root", Some(submatch)) => clg::command::root(submatch),
        _ => unreachable!(),
    };
    match result {
        Ok(code) => std::process::exit(code),
        Err(e) => {
            eprintln!("ERROR: {}", e);
            std::process::exit(1);
        }
    }
}
