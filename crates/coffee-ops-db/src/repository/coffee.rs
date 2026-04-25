use rootcause::Report;
use turso::transaction::Transaction;
use coffee_ops_core::domain::coffee::{Coffee, CoffeeId};
use coffee_ops_core::port::db::CoffeeRepository;

pub struct TursoCoffeeRepository<'tx> {
    transaction: &'tx Transaction<'tx>
}

impl CoffeeRepository for TursoCoffeeRepository<'_> {

    async fn insert(&self, coffee: &Coffee) -> Result<(), Report> {
        todo!()
    }

    async fn update(&self, coffee: &Coffee) -> Result<(), Report> {
        todo!()
    }

    async fn find_by_id(&self, id: &CoffeeId) -> Result<Option<Coffee>, Report> {
        todo!()
    }

    async fn find_all(&self) -> Result<Vec<Coffee>, Report> {
        todo!()
    }
}