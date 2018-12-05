use budget_lib::Budget;
use clap::{App, ArgMatches, SubCommand};

pub fn command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("add")
        // app settings

        // subcommands
}

pub fn delegate(budget: &mut Budget, matches: &ArgMatches) {
    process(budget, matches)
}

pub fn process(budget: &mut Budget, matces: &ArgMatches) {
    unimplemented!()
}