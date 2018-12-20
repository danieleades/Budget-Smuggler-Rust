use budget_lib::Budget;
use clap::{App, Arg, ArgMatches, SubCommand};

pub fn command<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("rename")
                .arg(Arg::with_name("old name").required(true).takes_value(true))
                .arg(Arg::with_name("new name").required(true).takes_value(true))
}

pub fn run(budget: &mut Budget, matches: &ArgMatches) {
        budget.rename_category(
                matches.value_of("old name").unwrap(),
                matches.value_of("new name").unwrap(),
        )
}
