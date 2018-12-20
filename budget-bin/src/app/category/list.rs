use budget_lib::Budget;
use clap::{App, ArgMatches, SubCommand};

pub fn command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("list")
    // app settings

    // subcommands
}

pub fn run(budget: &mut Budget, _matches: &ArgMatches) {
    for category in budget.categories() {
        println!("{}", category.name());
    }
}
