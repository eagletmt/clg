pub fn build_cli() -> clap::App<'static, 'static> {
    clap::App::new("clg")
        .version(clap::crate_version!())
        .setting(clap::AppSettings::SubcommandRequired)
        .subcommand(
            clap::SubCommand::with_name("clone")
                .arg(
                    clap::Arg::with_name("name")
                        .short("-n")
                        .long("name")
                        .takes_value(true)
                        .help("Change name of local repository"),
                )
                .arg(clap::Arg::with_name("URL").required(true).takes_value(true)),
        )
        .subcommand(
            clap::SubCommand::with_name("look").arg(
                clap::Arg::with_name("REPOSITORY")
                    .required(true)
                    .takes_value(true),
            ),
        )
        .subcommand(
            clap::SubCommand::with_name("list").arg(
                clap::Arg::with_name("completion")
                    .long("completion")
                    .help("Generate repository list for completion"),
            ),
        )
        .subcommand(clap::SubCommand::with_name("root").help("Print clg root directory"))
}
