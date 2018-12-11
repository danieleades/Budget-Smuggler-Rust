#![warn(clippy::all)]

mod app;
mod logging;
mod sd;
use budget_lib::Budget;

fn main() {
    let budget_root = dirs::home_dir()
        .expect("unable to determine home directory")
        .join(".budget");

    // create the Budget directory if it doesn't already exist
    std::fs::DirBuilder::new()
        .recursive(true)
        .create(&budget_root)
        .unwrap_or_else(|e| {
            panic!(
                "unable to create budget directory {:?}, error: {}",
                &budget_root, e
            )
        });

    // set up logging for the app
    logging::setup_logging(&budget_root.join("log"));

    // load the budget from the budget directory, or create a new one
    //let mut budget = Budget::from_directory(&budget_root).unwrap_or_default();
    let mut budget: Budget = sd::from_file(&budget_root).unwrap_or_default();

    app::run(&mut budget);

    //budget.save_to_directory(&budget_root).unwrap();
    sd::to_file(&budget, &budget_root).unwrap();
}
