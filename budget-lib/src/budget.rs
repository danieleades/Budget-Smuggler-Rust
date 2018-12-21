use crate::transaction::Transaction;
use std::iter::FromIterator;
use decimal::d128;
use std::collections::BTreeMap;
use crate::currency::Currency;
use crate::summary::Summary;
use crate::month::CalendarMonth;
use crate::ledger::Ledger;
use serde_derive::{Serialize, Deserialize};

/// A Budget, as the name suggests, represents an entire
/// budget for a single account.
/// 
/// For performance, the Budget caches summaries of the transactions
/// by month. When a new transaction is added, the cached summaries are simply
/// updated. This avoids traversing the entire list of transactions (unless
/// absolutely necessary).
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Budget<C> where C: Currency {
    transactions: Ledger<C>,
    cached_summaries: BTreeMap<CalendarMonth, Summary<C>>,
}

impl<C> Budget<C> where C: Currency {
    pub fn add(&mut self, t: Transaction<C>) {
        let m: CalendarMonth = t.date().into();
        self.cached_summaries.entry(m).or_default().add(&t);
        self.transactions.add(t);
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
            b.cached_summaries.entry(m).or_default().add(&transaction);
            b.transactions.add(transaction);
        }

        b
    }
}