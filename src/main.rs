// use {
    // argh::FromArgs,
// };

mod commands;

fn main() {
    let args: commands::Commands = argh::from_env();
    commands::run_subcommand(args);
    // args.command.run();
    // println!("{:#?}", args);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}