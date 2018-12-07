use budget_lib::Budget;
use clap::{crate_authors, crate_version, App, AppSettings, ArgMatches};
mod transaction;
mod category;

pub fn run(budget: &mut Budget) {
    let app = get_app();
    let matches = app.get_matches();
    delegate(budget, &matches);
}

fn get_app<'a, 'b>() -> App<'a, 'b> {
    App::new("Budget-Smuggler")
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .before_help("Personal finance and budgeting app.")
        // app settings
        .setting(AppSettings::AllowNegativeNumbers)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::GlobalVersion)
        // subcommands
        .subcommand(transaction::command())
        .subcommand(category::command())
}

fn delegate(budget: &mut Budget, matches: &ArgMatches) {
    match matches.subcommand() {
        ("transaction", Some(submatches)) => transaction::delegate(budget, submatches),
        ("category", Some(submatches)) => category::delegate(budget, submatches),
        //assume 'transaction'
        (_, None) => transaction::delegate(budget, matches),
        _ => panic!("app::delegate not implemented correctly!"),
    }
}
