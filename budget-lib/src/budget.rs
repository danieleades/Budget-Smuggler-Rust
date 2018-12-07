use crate::serialise::{deserialise_from_file, serialise_to_file, SerialiseError};
use crate::{types::CalendarMonth, Ledger, Transaction};
use chrono::Datelike;
use decimal::d128;
use serde_derive::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use uuid::Uuid;

type CategoryID = Uuid;

#[derive(Default, Debug)]
pub struct Budget {
    /// a list of the transactions that make up the budget
    transactions: Ledger,

    master_categories: HashMap<CategoryID, MasterCategory>,

    categories: Categories,

    /// The allocations are the amounts budgeted for each category for a given month
    allocations: BTreeMap<(CalendarMonth, CategoryID), Allocation>,

    /// A map of transactions summaries. The key is a tuple of Calendar Month, and a category ID.
    summaries: BTreeMap<(CalendarMonth, CategoryID), Summary>,
    uncategorised_summaries: BTreeMap<CalendarMonth, Summary>,
}

impl Budget {
    pub fn from_directory<P: AsRef<Path>>(path: P) -> Result<Self, SerialiseError> {
        let transactions = deserialise_from_file(path.as_ref().join("ledger"))?;
        let (master_categories, categories, allocations, summaries, uncategorised_summaries) =
            deserialise_from_file(path.as_ref().join("budget"))?;
        Ok(Budget {
            transactions,
            master_categories,
            categories,
            allocations,
            summaries,
            uncategorised_summaries,
        })
    }

    pub fn save_to_directory<P: AsRef<Path>>(&self, path: P) -> Result<(), SerialiseError> {
        serialise_to_file(&self.transactions, path.as_ref().join("ledger"))?;
        serialise_to_file(
            &(
                &self.master_categories,
                &self.categories,
                &self.allocations,
                &self.summaries,
                &self.uncategorised_summaries,
            ),
            path.as_ref().join("budget"),
        )?;
        Ok(())
    }

    pub fn master_categories(&self) -> impl Iterator<Item = &MasterCategory> {
        self.master_categories.values()
    }
    pub fn categories(&self) -> impl Iterator<Item = &Category> {
        self.categories.values()
    }

    pub fn ledger(&self) -> &Ledger {
        &self.transactions
    }

    pub fn add(&mut self, t: Transaction) {
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
        amount: impl Into<d128>,
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
struct Category {
    name: String,
    sort: i32,
    hidden: bool,
}

impl Category {
    fn new<S: Into<String>>(name: S) -> Self {
        Category {
            name: name.into(),
            sort: 0,
            hidden: false,
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct Allocation {
    amount: d128,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct Summary {
    n: u32,
    sum: d128,
    sum_squared: d128,
}

impl Summary {
    fn add(&mut self, t: &Transaction) {
        self.n += 1;
        self.sum += t.amount();
        self.sum_squared += t.amount() * t.amount();
    }
}

impl From<Ledger> for Budget {
    fn from(ledger: Ledger) -> Budget {
        let mut budget = Budget::default();

        for transaction in ledger {
            budget.add(transaction);
        }

        budget
    }
}

impl From<Budget> for Ledger {
    fn from(budget: Budget) -> Ledger {
        budget.transactions
    }
}
