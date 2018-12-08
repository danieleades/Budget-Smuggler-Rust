use budget_lib::Budget;
use clap::{App, ArgMatches, SubCommand};

mod add;
mod list;

pub fn command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("transaction")
        // app settings
        // subcommands
        .subcommand(add::command())
        .subcommand(list::command())
}

pub fn delegate(budget: &mut Budget, matches: &ArgMatches) {
    match matches.subcommand() {
        ("add", Some(submatches)) => add::delegate(budget, submatches),
        ("list", Some(submatches)) => list::delegate(budget, submatches),
        // If no subcommand is found, assume 'list'
        (_, None) => list::delegate(budget, matches),
        _ => panic!("something is missing!"),
    }
}
