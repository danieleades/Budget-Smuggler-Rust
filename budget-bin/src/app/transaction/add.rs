use budget_lib::{Budget, Transaction};
use clap::{App, Arg, ArgMatches, SubCommand};
use decimal::d128;
use std::str::FromStr;

pub fn command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("add")
        // app settings
        // arguments
        .arg(
            Arg::with_name("amount")
                .takes_value(true)
                .help("The amount of the transaction")
                .required(true),
        )
}

pub fn delegate(budget: &mut Budget, matches: &ArgMatches) {
    process(budget, matches)
}

pub fn process(budget: &mut Budget, matches: &ArgMatches) {
    let amount = d128::from_str(matches.value_of("amount").unwrap()).unwrap();
    budget.add(Transaction::new(amount))
}
