use {
    argh::FromArgs,
  };

#[derive(FromArgs, PartialEq, Debug)]
/// Run subcommand.
#[argh(subcommand, name = "dump")]
pub struct Cmd {
    #[argh(positional)]
    /// the file to run
    file: String,
}

impl Cmd {
    pub fn run(&self) {
        println!("DUMP - {:#?}", self.file);
    }
}