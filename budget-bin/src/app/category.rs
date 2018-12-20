use budget_lib::Budget;
use clap::{App, AppSettings, ArgMatches, SubCommand};

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

pub fn run(budget: &mut Budget, matches: &ArgMatches) {
        match matches.subcommand() {
                ("add", Some(submatches)) => add::run(budget, submatches),
                ("list", Some(submatches)) => list::run(budget, submatches),
                ("rename", Some(submatches)) => rename::run(budget, submatches),
                // If no subcommand is found, assume 'list'
                (_, None) => list::run(budget, matches),
                _ => panic!("something is missing!"),
        }
}
