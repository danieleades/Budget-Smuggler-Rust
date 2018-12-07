mod logging;
mod app;
use budget_lib::Budget;

fn main() {
    let budget_root = dirs::home_dir()
        .expect("unable to determine home directory")
        .join(".budget");

    // create the Budget directory if it doesn't already exist
    std::fs::DirBuilder::new()
        .recursive(true)
        .create(&budget_root)
        .unwrap_or_else(|_| panic!("unable to create budget directory: {:?}", &budget_root));
    
    // set up logging for the app
    logging::setup_logging(&budget_root.join("log"));

    // load the budget from the budget directory, or create a new one
    let mut budget = Budget::from_directory(&budget_root).unwrap_or_default();

    app::run(&mut budget);

    budget.save_to_directory(&budget_root).unwrap();
}
