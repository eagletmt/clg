fn main() -> Result<(), anyhow::Error> {
    env_logger::init();
    let app = clg::cli::build_cli();

    match app.get_matches().subcommand() {
        ("clone", Some(submatch)) => clg::command::clone(submatch),
        ("look", Some(submatch)) => clg::command::look(submatch),
        ("list", Some(submatch)) => clg::command::list(submatch),
        ("root", Some(submatch)) => clg::command::root(submatch),
        _ => unreachable!(),
    }?;
    Ok(())
}
