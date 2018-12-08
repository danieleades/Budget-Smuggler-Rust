use budget_lib::Budget;
use clap::{App, ArgMatches, SubCommand, AppSettings};

mod add;
mod list;
mod rename;

pub fn command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("category")
        // app settings
        .setting(AppSettings::SubcommandRequiredElseHelp)
        // subcommands
       .subcommand(add::command())
       .subcommand(list::command())
       .subcommand(rename::command())
}

pub fn delegate(budget: &mut Budget, matches: &ArgMatches) {
    match matches.subcommand() {
        ("add", Some(submatches)) => add::delegate(budget, submatches),
        ("list", Some(submatches)) => list::delegate(budget, submatches),
        ("rename", Some(submatches)) => rename::delegate(budget, submatches),
        // If no subcommand is found, assume 'list'
        (_, None) => list::delegate(budget, matches),
        _ => panic!("something is missing!"),
    }
}