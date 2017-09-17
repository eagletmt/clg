extern crate clap;

pub fn build_cli() -> clap::App<'static, 'static> {
    clap::App::new("clg")
        .version(crate_version!())
        .setting(clap::AppSettings::SubcommandRequired)
        .subcommand(clap::SubCommand::with_name("clone").arg(
            clap::Arg::with_name("URL").required(true).takes_value(true),
        ))
        .subcommand(
            clap::SubCommand::with_name("look").arg(
                clap::Arg::with_name("REPOSITORY")
                    .required(true)
                    .takes_value(true),
            ),
        )
        .subcommand(clap::SubCommand::with_name("list"))
}
