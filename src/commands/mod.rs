use argh::FromArgs;

pub mod dump;

#[derive(FromArgs, PartialEq, Debug)]
/// Top-level command.
pub struct Commands {
    #[argh(subcommand)]
    command: Subcommands,

    #[argh(switch, short = 'v', description = "whether to have verbose output")]
    verbose: bool,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Subcommands {
    Dump(dump::Cmd),
}

pub fn run_subcommand(args: Commands) {
    match args.command {
        Subcommands::Dump(cmd) => cmd.run(),
    }
}
