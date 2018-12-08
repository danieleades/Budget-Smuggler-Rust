# Next Steps
1. add a command for allocating funds to a category. maybe 'transfer'?
2. make a nice pretty summary table
3. flesh out the existing commands a little. For example transaction.add needs a bunch more arguments for adding other transaction details
4. add a whole lot more logging to both the app and the library- this will make it much easier to debug down the track
5. UNIT TESTS!!
6. library documentation (documentation comments start with `///`)
7. write out a bunch of syntax for various commands. (what is the least annoying way to use this app?)

## Table Format
messing about to figure out a nice format for a budget summary table

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