use crate::Currency;
use crate::{CalendarMonth, Ledger, Transaction};
use chrono::Datelike;
use decimal::d128;
use serde_derive::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};
use uuid::Uuid;

type CategoryID = Uuid;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Budget<C = d128>
where
    C: Currency,
{
    /// a list of the transactions that make up the budget
    transactions: Ledger<C>,

    master_categories: HashMap<CategoryID, MasterCategory>,

    categories: Categories,

    /// The allocations are the amounts budgeted for each category for a given month
    allocations: BTreeMap<(CalendarMonth, CategoryID), Allocation<C>>,

    /// A map of transactions summaries. The key is a tuple of Calendar Month, and a category ID.
    summaries: BTreeMap<(CalendarMonth, CategoryID), Summary<C>>,
    uncategorised_summaries: BTreeMap<CalendarMonth, Summary<C>>,
}

impl<C> Budget<C>
where
    C: Currency,
{
    pub fn master_categories(&self) -> impl Iterator<Item = &MasterCategory> {
        self.master_categories.values()
    }
    pub fn categories(&self) -> impl Iterator<Item = &Category> {
        self.categories.values()
    }

    pub fn add_category<'a, S>(&mut self, name: S) -> Result<&str, &str>
    where
        S: Into<Cow<'a, str>>,
    {
        let n = name.into();
        match self.categories.get_id(&n) {
            Some(id) => Err(&self.categories.get(&id).unwrap().name),
            None => {
                let id = self.categories.get_or_create_id(n);
                Ok(&self.categories.get(&id).unwrap().name)
            }
        }
    }

    pub fn ledger(&self) -> &Ledger<C> {
        &self.transactions
    }

    pub fn add(&mut self, t: Transaction<C>) {
        let date: CalendarMonth = t.date().into();

        if let Some(name) = t.category() {
            let id = self.categories.get_or_create_id(name);
            self.summaries.entry((date, id)).or_default().add(&t);
        } else {
            self.uncategorised_summaries
                .entry(date)
                .or_default()
                .add(&t);
        }

        self.transactions.add(t);
    }

    pub fn transfer<'a, S>(
        &mut self,
        amount: impl Into<C>,
        from_category: S,
        to_category: S,
        date: impl Datelike,
    ) -> Result<(), ()>
    where
        S: Into<Cow<'a, str>>,
    {
        let month: CalendarMonth = date.into();
        let a = amount.into();
        let from_id = self.categories.get_or_create_id(from_category);
        let to_id = self.categories.get_or_create_id(to_category);
        self.allocations
            .entry((month.clone(), from_id))
            .or_default()
            .amount -= a;
        self.allocations.entry((month, to_id)).or_default().amount += a;

        Ok(())
    }

    pub fn rename_category<'a, S>(&mut self, old_name: S, new_name: S)
    where
        S: Into<Cow<'a, str>>,
    {
        let old = old_name.into();
        let new = new_name.into();

        if let Some(x) = self
            .categories
            .values_mut()
            .find(|x| x.name == old.as_ref())
        {
            x.name = new.to_string();
        }

        if let Some(t) = (&mut self.transactions)
            .into_iter()
            .find(|t| t.category() == &Some(old.to_string()))
        {
            t.set_category(Some(new))
        };
    }
}

#[derive(Default, Serialize, Deserialize, Debug)]
struct Categories {
    #[serde(flatten)]
    categories: HashMap<CategoryID, Category>,
}

impl std::ops::Deref for Categories {
    type Target = HashMap<CategoryID, Category>;
    fn deref(&self) -> &Self::Target {
        &self.categories
    }
}

impl std::ops::DerefMut for Categories {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.categories
    }
}

impl Categories {
    fn get_or_create_id<'a, S>(&mut self, name: S) -> CategoryID
    where
        S: Into<Cow<'a, str>>,
    {
        let n = name.into();
        if let Some(id) = self.get_id(&n) {
            *id
        } else {
            let id = CategoryID::new_v4();
            self.insert(id, Category::new(n));
            id
        }
    }

    fn get_id<S: AsRef<str>>(&self, name: S) -> Option<&CategoryID> {
        self.iter().find(|x| x.1.name == name.as_ref()).map(|x| x.0)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct MasterCategory {
    name: String,
    sort: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    name: String,
    sort: i32,
    hidden: bool,
}

impl Category {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Category {
            name: name.into(),
            sort: 0,
            hidden: false,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct Allocation<C>
where
    C: Currency,
{
    amount: C,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct Summary<C>
where
    C: Currency,
{
    n: u32,
    sum: C,
}

impl<C> Summary<C>
where
    C: Currency,
{
    fn add(&mut self, t: &Transaction<C>) {
        self.n += 1;
        self.sum += *t.amount();
    }
}

impl<C> From<Ledger<C>> for Budget<C>
where
    C: Currency,
{
    fn from(ledger: Ledger<C>) -> Budget<C> {
        let mut budget = Budget::default();

        for transaction in ledger {
            budget.add(transaction);
        }

        budget
    }
}

impl<C> From<Budget<C>> for Ledger<C>
where
    C: Currency,
{
    fn from(budget: Budget<C>) -> Ledger<C> {
        budget.transactions
    }
}
