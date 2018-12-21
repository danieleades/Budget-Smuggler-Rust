use crate::category::Category;
use crate::currency::Currency;
use crate::transaction::Transaction;
use std::collections::HashMap;
use std::iter::FromIterator;
use serde_derive::{Serialize, Deserialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Summary<C>
where
    C: Currency,
{
    data: SummaryData<C>,
    subcategories: HashMap<String, Summary<C>>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SummaryData<C> {
    /// the number of transactions
    n: u32,

    /// sum of transactions
    sum: C,
}

impl<C> SummaryData<C>
where
    C: Currency,
{
    fn add(&mut self, t: &Transaction<C>) {
        self.n += 1;
        self.sum += *t.amount();
    }
}

impl<C> Summary<C>
where
    C: Currency,
{
    pub fn add(&mut self, t: &Transaction<C>) {
        self.add_recursive(t, 0);
    }

    fn add_recursive(&mut self, t: &Transaction<C>, depth: usize) {
        self.data.add(&t);

        if t.category().depth() > depth {
            let c = t.category()[depth].clone();
            self.subcategories
                .entry(c)
                .or_default()
                .add_recursive(t, depth + 1)
        }
    }

    /// Flatten a hierarchical Summary object into a vector of tuples containing
    /// the category name and the summary data.
    ///
    /// # Example
    /// ```
    /// println!("hello, doc test!");
    /// ```
    pub fn flatten(self) -> impl Iterator<Item = (Category, SummaryData<C>)> {
        self.flatten_recursive(Category::default()).into_iter()
    }

    fn flatten_recursive(self, c: Category) -> Vec<(Category, SummaryData<C>)> {
        let mut flattened_summaries = Vec::new();

        flattened_summaries.push((c.clone(), self.data));

        for (s, sub_summary) in self.subcategories {
            let subcategory = c.clone().with_subcategory(s);
            flattened_summaries.append(&mut sub_summary.flatten_recursive(subcategory));
        }

        flattened_summaries
    }
}

impl<C> FromIterator<Transaction<C>> for Summary<C>
where
    C: Currency,
{
    fn from_iter<I: IntoIterator<Item = Transaction<C>>>(iter: I) -> Self {
        let mut s = Summary::default();

        for transaction in iter {
            s.add(&transaction);
        }

        s
    }
}
