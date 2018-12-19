# Next Steps
1. make a nice pretty summary table
2. add some default sensible default categories
3. flesh out the existing commands a little. For example transaction.add needs a bunch more arguments for adding other transaction details. For starters, it needs arguments to add a category and a transaction date. probably something like-

`arg.(Arg::with_name("category").required(false).short("c").long("category").takes_value(true)`.

4. add a whole lot more logging to both the app and the library- this will make it much easier to debug down the track. Logging is done with macros from the 'log' crate. ie. `log::info!("...")`, `log::warn!("...")`, `log::error!("...")`, etc.
5. UNIT TESTS!! Need these for everything in the library
6. library documentation (documentation comments start with `///`). Documentation comments in Rust are sick. they get rendered really nicely if you run `cargo doc`, and any code examples in the comments actually get compiled and tested when you run `cargo test`.
At some point i'll add the compiler flag `#![deny(missing_docs)]` which causes builds to fail if anything is missing documentation comments.
7. write out a bunch of syntax for various commands. (what is the least annoying way to use this app?). I made a start on this but would really appreciate some input.

## Table Format
messing about to figure out a nice format for a budget summary table. This would be for a single month, but it might be nice to forecast the next couple of months as well. I've thrown in a screenshot from a budgeting app for reference too.

| Category      | rollover         | spent | budgeted | remaining |
| ------------- |:-------------:| -----:| --- | --- |
| **Income** | $0 |
| **Monthly Bills**     | | |
| Rent     | $0     |   $1000 | $1000 | $0
| Electricity | $15     |    $40 | $50 | $25
| **Everyday Expenses**    | | | |  
| Groceries | $15     |    $40 | $50 | $25
| Coffee    | $0     |   $30 | $25 | $5
| Clothes   | $0     |   - | - | -
| **Fun Stuff**    | | | |  
| Restaurants | $15     |  - | - | -

## Syntax
these are just various, possibly contradictory ideas
### add a transaction
current syntax:

`$ budget transaction add [amount] [description] --date [date] --category [category]`

less verbose syntax (use a category name to create a new transaction, assume transactions are outgoings):

`$ budget [amount] [category] --date [date]`

this will fail unless the category already exists, unless you pass a flag such as `--force` or `--create`.

to add income:

`$ budget in [amount]`

'`$ budget`' by itself is an alias for '`$ budget summary`'
