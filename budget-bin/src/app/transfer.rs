use crate::app::AppTools;
use budget_lib::Budget;
use clap::{App, Arg, ArgMatches, SubCommand};
use decimal::d128;

pub fn command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("transfer")
        // app settings
        // arguments
        .arg(
            Arg::with_name("amount")
                .takes_value(true)
                .help("The amount to transfer")
                .required(true),
        )
        .arg(
            Arg::with_name("from category")
                .takes_value(true)
                .help("The category to transfer funds out of")
                .required(true),
        )
        .arg(
            Arg::with_name("to category")
                .takes_value(true)
                .help("The category to transfer funds into")
                .required(true),
        )
}

pub fn delegate(budget: &mut Budget, matches: &ArgMatches) {
    process(budget, matches)
}

pub fn process(budget: &mut Budget, matches: &ArgMatches) {
    let amount: d128 = matches.typed_value_of("amount", None).unwrap();
    match budget.transfer(
        amount,
        matches.value_of("to category").unwrap(),
        matches.value_of("from category").unwrap(),
        chrono::Local::today(),
    ) {
        Ok(_) => (),
        Err(_) => println!("unable to transfer"),
    }
}
