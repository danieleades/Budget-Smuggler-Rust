use budget_lib::Budget;
use clap::{App, Arg, ArgMatches, SubCommand};

pub fn command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("add")
        // app settings
        // arguments
        .arg(
            Arg::with_name("name")
                .takes_value(true)
                .help("The name of the category")
                .required(true),
        )
}

pub fn run(budget: &mut Budget, matches: &ArgMatches) {
    let category_name = matches.value_of("name").unwrap();
    match budget.add_category(category_name) {
        Ok(n) => log::info!("Category added: {}", n),
        Err(n) => log::warn!("Category already exists: {}", n),
    }
}
