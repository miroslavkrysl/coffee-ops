use crate::domain::coffee::{Coffee, CoffeeId};
use rootcause::Report;

pub trait CoffeeRepository {
    async fn insert(&self, coffee: &Coffee) -> Result<(), Report>;

    async fn update(&self, coffee: &Coffee) -> Result<(), Report>;

    async fn find_by_id(&self, id: &CoffeeId) -> Result<Option<Coffee>, Report>;

    async fn find_all(&self) -> Result<Vec<Coffee>, Report>;
}
