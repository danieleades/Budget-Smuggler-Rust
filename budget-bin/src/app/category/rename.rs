use budget_lib::Budget;
use clap::{App, ArgMatches, SubCommand, Arg};

pub fn command<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("rename")
        .arg(Arg::with_name("old name").required(true).takes_value(true))
        .arg(Arg::with_name("new name").required(true).takes_value(true))
}

pub fn delegate(budget: &mut Budget, matches: &ArgMatches) {
        process(budget, matches)
}

pub fn process(budget: &mut Budget, matches: &ArgMatches) {
        budget.rename_category(matches.value_of("old name").unwrap(), matches.value_of("new name").unwrap())
}