use budget_lib::Budget;
use clap::{App, ArgMatches, SubCommand};

pub fn command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("list").about("Lists the transactions in the budget.")
    // app settings

    // subcommands
}

pub fn run(budget: &mut Budget, _matches: &ArgMatches) {
    println!("{:?}", budget.ledger());
}
