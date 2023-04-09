fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();
    use clap::Parser as _;
    let args = clg::cli::Args::parse();

    match args.command {
        clg::cli::Command::Clone(args) => clg::command::clone(args),
        clg::cli::Command::Look(args) => clg::command::look(args),
        clg::cli::Command::List(args) => clg::command::list(args),
        clg::cli::Command::Root => clg::command::root(),
    }?;
    Ok(())
}
