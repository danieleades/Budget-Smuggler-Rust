# Next Steps
1. make a nice pretty summary table
2. the budget struct has methods for saving to, and loading from a file. If these are going to be in the library, they should probably be behind a feature gate, as many consumers of the library won't use this, and it adds a lot of dependencies. This would be done with Rust's 'conditional compilation' feature.
3. flesh out the existing commands a little. For example transaction.add needs a bunch more arguments for adding other transaction details. For starters, it needs arguments to add a category and a transaction date. probably something like-

`arg.(Arg::with_name("category").required(false).short("c").long("category").takes_value(true)`.

4. add a whole lot more logging to both the app and the library- this will make it much easier to debug down the track. Logging is done with macros from the 'log' crate. ie. `log::info!("...")`, `log::warn!("...")`, `log::error!("...")`, etc.
5. UNIT TESTS!! Needs these for everything in the library
6. library documentation (documentation comments start with `///`). Documentation comments in Rust are sick. they get rendered really nicely if you run `cargo doc`, and any code examples in the comments actually get compiled and tested when you run `cargo test`.
At some point i'll add the compiler flag `#![deny(missing_docs)]` which causes builds to fail if anything is missing documentation comments.
7. write out a bunch of syntax for various commands. (what is the least annoying way to use this app?). I made a start on this but would really appreciate some input.
8. i've used a 128 bit decimal type (d128) for transaction amounts, as this type doesn't have the rounding problems that you get representing currency with floats, or the loss of significance issues with integers. The downside is that basic operations are much slower. It might be worth templating the transaction, ledger, and budget types to be generic over any type that supports basic operations, with a default generic type of d128. That way a consumer of the library could use floats under the hood if they needed greater speed and perfect accuracy wasn't a consideration.

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