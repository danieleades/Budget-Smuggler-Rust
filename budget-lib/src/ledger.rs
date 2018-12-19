use crate::Currency;
use crate::Transaction;
use decimal::d128;
use serde_derive::{Deserialize, Serialize};

// this is an intentially simplistic collection type. Once I've been using this for a while, and know what i actually need it to be able to do, then I
// can start optimising it and getting clever about it.

/// A Ledger represents a collection of Transactions.
///
/// Ledger offers some useful helper methods for
/// working with Transactions.
#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Ledger<C = d128>
where
    C: Currency,
{
    transactions: Vec<Transaction<C>>,
}

impl<C> Ledger<C>
where
    C: Currency,
{
    /// Create a new Ledger from an iterator over Transactions.
    ///
    /// # Example
    /// ```
    /// use budget_lib::{Transaction, Ledger};
    ///
    /// let v = vec![
    ///     Transaction::new(10),
    ///     Transaction::new(20),
    ///     Transaction::new(30),
    /// ];
    ///
    /// let l = Ledger::from_transactions(
    ///     v.into_iter()
    /// );
    /// ```
    pub fn from_transactions<T>(transactions: T) -> Self
    where
        T: IntoIterator<Item = Transaction<C>>,
    {
        let mut ledger = Ledger {
            transactions: transactions.into_iter().collect(),
        };
        ledger.sort_by_date();
        ledger
    }

    fn sort_by_date(&mut self) {
        self.transactions.sort_by_key(Transaction::<C>::date)
    }

    pub fn categories(&self) -> Vec<String> {
        let categories: Vec<String> = self
            .transactions
            .iter()
            .map(|x| x.category())
            .filter(|x| x.is_some())
            .map(|x| x.clone().unwrap())
            .collect();
        categories
    }

    pub fn add(&mut self, t: Transaction<C>) {
        self.transactions.push(t);
        self.sort_by_date();
    }
}

impl<C> IntoIterator for Ledger<C>
where
    C: Currency,
{
    type Item = Transaction<C>;
    type IntoIter = std::vec::IntoIter<Transaction<C>>;

    fn into_iter(self) -> Self::IntoIter {
        self.transactions.into_iter()
    }
}

impl<'a, C> IntoIterator for &'a Ledger<C>
where
    C: Currency,
{
    type Item = &'a Transaction<C>;
    type IntoIter = std::slice::Iter<'a, Transaction<C>>;

    fn into_iter(self) -> Self::IntoIter {
        self.transactions.iter()
    }
}

impl<'a, C> IntoIterator for &'a mut Ledger<C>
where
    C: Currency,
{
    type Item = &'a mut Transaction<C>;
    type IntoIter = std::slice::IterMut<'a, Transaction<C>>;
    fn into_iter(self) -> Self::IntoIter {
        self.transactions.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::{d128, Ledger, Transaction};

    #[test]
    fn default_to_d128() {
        let d = d128::zero();
        let t = Transaction::default();
        assert_eq!(&d, t.amount());
    }

    #[test]
    fn constructors() {
        let t1 = Transaction::<d128>::default();
        let mut ledger_d128 = Ledger::default();
        ledger_d128.add(t1);

        let t2 = Transaction::<f32>::default();
        let mut ledger_f32 = Ledger::default();
        ledger_f32.add(t2);
    }
}
