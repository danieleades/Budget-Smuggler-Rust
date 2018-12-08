use budget_lib::{Budget, Transaction};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use decimal::d128;
use std::str::FromStr;

pub fn command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("add")
        .setting(AppSettings::TrailingVarArg)
        // app settings
        // arguments
        .arg(
            Arg::with_name("amount")
                .takes_value(true)
                .help("The amount of the transaction")
                .required(true),
        )
        .arg(Arg::with_name("description").multiple(true))
}

pub fn delegate(budget: &mut Budget, matches: &ArgMatches) {
    process(budget, matches)
}

pub fn process(budget: &mut Budget, matches: &ArgMatches) {
    let amount = d128::from_str(matches.value_of("amount").unwrap()).unwrap();
    let description = matches.values_of("description").map(collect_sentence);

    budget.add(Transaction::new(amount).with_description(description))
}

fn collect_sentence<'a>(mut tokens: impl Iterator<Item = &'a str>) -> String {
    let mut sentence = "".to_string();
    if let Some(x) = tokens.next() {
        sentence.push_str(&x);
    }
    for token in tokens {
        sentence += &format!(" {}", token)
    }
    sentence
}
