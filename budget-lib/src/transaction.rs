//! # Transaction.

use crate::category::Category;
use crate::Currency;
use chrono::{Date, DateTime, Utc};
use decimal::d128;
use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

/// A struct which represents a financial transaction.
/// Most of the fields are optional, and will only be
/// serialised if present.
///
/// Creating a Transaction with its 'new' method will
/// result in a Transaction<d128>. d128 is a 128 bit
/// decimal type. This type doesn't suffer the same
/// rounding and loss of significance errors as
/// float and integer types, at the cost of the
/// speed of operations.
///
/// For faster operations, Transaction provides the
/// 'with_currency' method to use another type to
/// represent the transaction amount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction<C>
where
    C: Currency,
{
    /// transaction value. a positive number represents flow into the account
    amount: C,

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    payee: Option<String>,

    /// the date that the transaction is created. If no transaction date is set, this will be used for sorting
    date_created: DateTime<Utc>,

    /// the date that the transaction occurred
    #[serde(skip_serializing_if = "Option::is_none")]
    date_transaction: Option<DateTime<Utc>>,

    /// An optional category for the transaction
    #[serde(skip_serializing_if = "Category::is_empty")]
    category: Category,

    #[serde(skip_serializing_if = "Option::is_none")]
    account: Option<String>,

    /// A vector of strings used to organise transactions
    tags: Vec<String>,

    /// An optional non-unique id
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<u16>,

    /// A globally unique id
    uuid: Uuid,

    /// If true, the budget has been reconciled past the date of this transaction. reconciled transactions should not be edited (lightly)
    reconciled: bool,

    /// The source of this transaction. This enum may be used for differentiating between transactions
    /// in a single ledger that came from different sources
    source: Source,
}

impl<C> Default for Transaction<C>
where
    C: Currency,
{
    fn default() -> Self {
        Transaction {
            amount: C::default(),
            description: None,
            payee: None,
            date_created: Utc::now(),
            date_transaction: None,
            category: Category::default(),
            account: None,
            tags: Vec::<String>::default(),
            id: None,
            uuid: Uuid::new_v4(),
            reconciled: false,
            source: Source::Manual,
        }
    }
}

impl Transaction<d128> {
    /// Create a new Transaction<d128>
    /// ```
    /// use decimal::d128;
    /// use budget_lib::Transaction;
    ///
    /// let t = Transaction::new(100);
    ///
    /// assert_eq!(t.amount(), &d128::from(100));
    /// ```
    pub fn new<T: Into<d128>>(amount: T) -> Self {
        Self {
            amount: amount.into(),
            ..Self::default()
        }
    }
}

impl<C> Transaction<C>
where
    C: Currency,
{
    /// Create a new Transaction with a
    /// custom type to represent the transaction
    /// amount. Any type can be used provided it
    /// implements a few basic operations.
    /// ```
    /// use budget_lib::Transaction;
    ///
    /// let amount: f32 = 100.0;
    /// let t = Transaction::with_currency(amount);
    ///
    /// assert_eq!(t.amount(), &amount);
    /// ```
    pub fn with_currency(amount: C) -> Self {
        Self {
            amount,
            ..Self::default()
        }
    }
    /// Return a reference to the amount of the transaction.
    pub fn amount(&self) -> &C {
        &self.amount
    }

    /// Set the amount of the transaction
    pub fn set_amount<T: Into<C>>(&mut self, amount: T) {
        self.amount = amount.into();
    }

    /// Set the amount of the transaction.
    ///
    /// This method consumes and returns the Transaction. This
    /// is useful for builder-style inline construction.
    ///
    /// # Example
    /// ```
    /// use budget_lib::Transaction;
    /// use decimal::d128;
    ///
    /// let t = Transaction::<d128>::default()
    ///             .with_amount(100);
    /// ```
    pub fn with_amount<T: Into<C>>(mut self, amount: T) -> Self {
        self.set_amount(amount.into());
        self
    }

    /// Returns the date and time that the Transaction was created.
    ///
    /// Setting the date and time that a transaction took place is
    /// optional. In the case that this is not set, it is useful to
    /// know the date that it was created.
    pub fn created(&self) -> DateTime<Utc> {
        self.date_created
    }

    /// Returns the date that the transaction occurred in real life.
    ///
    /// Since this so-called 'value date' of the transaction may not
    /// be set, this method returns an Option<DateTime<Utc>>.
    pub fn date_transaction(&self) -> Option<DateTime<Utc>> {
        self.date_transaction
    }

    /// Set the 'value date' of the transaction.
    ///
    /// This is the date that the transaction occurred in real life.
    /// This method accepts any type that can be converted into a
    /// chrono::DateTime<Utc>, which in practice is generally a
    /// chrono::DateTime<Utc>.
    ///
    /// This method accepts an Option, since the field is optional.
    /// The field can be cleared by passing in 'None'.
    ///
    /// # Example
    /// ```
    /// use chrono::{DateTime, Utc};
    /// use budget_lib::Transaction;
    ///
    /// let mut t = Transaction::new(100);
    /// t.set_date_transaction(Some(Utc::now()));
    ///
    /// assert!(t.date_transaction().is_some());
    ///
    /// let date_time: Option<DateTime<Utc>> = None;
    ///
    /// t.set_date_transaction(date_time);
    ///
    /// assert!(t.date_transaction().is_none());
    /// ```
    pub fn set_date_transaction(&mut self, date: Option<DateTime<Utc>>) {
        self.date_transaction = date;
    }

    /// Inline method for setting the date of the transaction (see 'date_transaction(...)').
    ///
    /// This method doesn't accept an option, since it is primarily used in
    /// constructing a new transaction, where the default transaction value
    /// date is `None`.
    ///
    /// # Example
    /// ```
    /// use chrono::{DateTime, Utc};
    /// use budget_lib::Transaction;
    ///
    /// let mut t = Transaction::new(100)
    ///                 .with_date_transaction(Utc::now());
    /// ```
    pub fn with_date_transaction(mut self, date: DateTime<Utc>) -> Self {
        self.set_date_transaction(Some(date));
        self
    }

    /// Returns the datetime of the transaction if set, otherwise this returns the
    /// datetime that the transaction was created.
    ///
    /// This is useful for sorting purposes. The 'value' date of the transaction is
    /// preferred, but not always present. For most applications its useful to
    /// assume that the transaction occurred around about the time the record was
    /// created.
    pub fn date(&self) -> DateTime<Utc> {
        self.date_transaction().unwrap_or_else(|| self.created())
    }

    /// Returns the transaction description, if present.
    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    /// Set the transaction description.
    ///
    /// If no description is set, this method
    /// will return None.
    pub fn set_description<S: Into<String>>(&mut self, description: Option<S>) {
        self.description = description.map(S::into);
    }

    /// Inline method for setting the description
    /// of a Transaction.
    ///
    /// # Example
    /// ```
    /// use budget_lib::Transaction;
    ///
    /// let t = Transaction::new(20).with_description(Some(
    ///             "today I found $20 it was the best day ever the end")
    ///         );
    ///
    /// assert!(t.description().is_some());
    /// ```
    pub fn with_description<S: Into<String>>(mut self, description: Option<S>) -> Self {
        self.description = description.map(S::into);
        self
    }

    /// Returns the payee of the transaction, if set.
    pub fn payee(&self) -> &Option<String> {
        &self.payee
    }

    pub fn set_payee<S: Into<String>>(&mut self, payee: Option<S>) {
        self.payee = payee.map(S::into);
    }

    pub fn with_payee<S: Into<String>>(mut self, payee: Option<S>) -> Self {
        self.payee = payee.map(S::into);
        self
    }

    pub fn category(&self) -> &Category {
        &self.category
    }

    pub fn set_category(&mut self, category: Option<Category>) {
        self.category = match category {
            Some(c) => c,
            None => Category::default(),
        }
    }

    pub fn set_category_from_str<S: AsRef<str>>(&mut self, category: S) {
        self.category = Category::from_str(category.as_ref()).unwrap_or_default();
    }

    pub fn with_category<S: Into<String>>(mut self, category: Option<Category>) -> Self {
        self.set_category(category);
        self
    }

    pub fn with_category_from_str<S: AsRef<str>>(mut self, category: S) -> Self {
        self.set_category_from_str(category);
        self
    }

    pub fn account(&self) -> &Option<String> {
        &self.account
    }

    pub fn set_account<S: Into<String>>(&self) -> &Option<String> {
        &self.account
    }

    /// add tag to transaction, if its not already present
    pub fn tag<S: Into<String>>(&mut self, tag: S) {
        let t: String = tag.into();
        if !self.tags.contains(&t) {
            self.tags.push(t)
        }
    }

    pub fn with_tag<S: Into<String>>(mut self, tag: S) -> Self {
        self.tag(tag);
        self
    }

    /// removes a tag, if it exists
    pub fn untag<S: AsRef<String>>(&mut self, tag: S) {
        self.tags.retain(|x| x != tag.as_ref());
    }

    pub fn without_tag<S: AsRef<String>>(mut self, tag: S) -> Self {
        self.untag(tag);
        self
    }

    /// sets the transaction tags to exactly those supplied
    pub fn set_tags<S: Into<String>>(&mut self, tags: Vec<S>) {
        let mut t: Vec<String> = tags.into_iter().map(S::into).collect();
        t.sort();
        t.dedup();
        self.tags = t;
    }

    pub fn tags(&self) -> std::slice::Iter<String> {
        self.tags.iter()
    }

    pub fn id(&self) -> Option<u16> {
        self.id
    }

    pub fn set_id<T: Into<u16>>(&mut self, id: Option<T>) {
        self.id = id.map(T::into);
    }

    pub fn with_id<T: Into<u16>>(mut self, id: Option<T>) -> Self {
        self.set_id(id);
        self
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn reconciled(&self) -> bool {
        self.reconciled
    }

    pub fn set_reconciled(&mut self, b: bool) {
        self.reconciled = b;
    }

    pub fn source(&self) -> &Source {
        &self.source
    }

    pub fn set_source(&mut self, s: Source) {
        self.source = s;
    }

    pub fn with_source(mut self, s: Source) -> Self {
        self.set_source(s);
        self
    }

    /// returns true if two transactions have the same amount, description, category, tags, transaction date.
    /// ids, added date, source, and reconciled state are not considered.
    pub fn is_similar(&self, other: &Transaction<C>) -> bool {
        self.amount() == other.amount()
            && self.description() == other.description()
            && self.category() == other.category()
            && self.date_transaction() == other.date_transaction()
            && self.tags().as_slice() == other.tags().as_slice()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Source {
    Manual,
    Reconciliation,
}

#[cfg(test)]
mod tests {
    use super::{d128, Transaction};

    #[test]
    fn constructors() {
        Transaction::<d128>::default();
        Transaction::<f64>::default();
    }
}
