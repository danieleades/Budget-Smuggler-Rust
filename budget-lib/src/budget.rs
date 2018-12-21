use crate::currency::Currency;
use crate::ledger::Ledger;
use crate::month::CalendarMonth;
use crate::summary::Summary;
use crate::transaction::Transaction;
use decimal::d128;
use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::iter::FromIterator;

/// A Budget, as the name suggests, represents an entire
/// budget for a single account.
///
/// For performance, the Budget caches summaries of the transactions
/// by month. When a new transaction is added, the cached summaries are simply
/// updated. This avoids traversing the entire list of transactions (unless
/// absolutely necessary).
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Budget<C>
where
    C: Currency,
{
    transactions: Ledger<C>,
    cache: Cache<C>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Cache<C>
where
    C: Currency,
{
    cached_summaries: BTreeMap<CalendarMonth, Summary<C>>,
}

impl<C> Budget<C>
where
    C: Currency,
{
    pub fn new(transactions: Ledger<C>, cache: Cache<C>) -> Self {
        Budget {
            transactions, cache,
        }
    }
    pub fn add(&mut self, t: Transaction<C>) {
        let m: CalendarMonth = t.date().into();
        self.cache.cached_summaries.entry(m).or_default().add(&t);
        self.transactions.add(t);
    }

    pub fn ledger(&self) -> &Ledger<C> {
        &self.transactions
    }

    pub fn cache(&self) -> &Cache<C> {
        &self.cache
    }
}

impl<C> FromIterator<Transaction<C>> for Budget<C>
where
    C: Currency,
{
    fn from_iter<I: IntoIterator<Item = Transaction<C>>>(iter: I) -> Self {
        let mut b = Budget::default();

        for transaction in iter {
            let m: CalendarMonth = transaction.date().into();
            b.cache.cached_summaries.entry(m).or_default().add(&transaction);
            b.transactions.add(transaction);
        }

        b
    }
}
