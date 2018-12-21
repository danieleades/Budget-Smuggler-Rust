use budget_lib::{Transaction, Summary, SummaryData, Category};
use decimal::d128;

fn main() {
    println!("hello, world!");

    let transactions = vec![
Transaction::new(-10).with_category_from_str("expenses"),
Transaction::new(-20).with_category_from_str("expenses::car"),
Transaction::new(-15).with_category_from_str("expenses::car::fuel"),
Transaction::new(50).with_category_from_str("income"),
    ];
    
    let s: Summary<d128> = transactions.into_iter().collect();

    println!("{:#?}", s);

    for line in s.flatten() {
        println!("\n{:?}", line);
    }

}