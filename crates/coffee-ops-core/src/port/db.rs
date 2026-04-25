use crate::domain::coffee::{Coffee, CoffeeId};
use rootcause::Report;

pub trait UnitOfWork {
    type MetadataRepository<'a>: MetadataRepository
    where
        Self: 'a;

    type CoffeeRepository<'a>: CoffeeRepository
    where
        Self: 'a;

    fn metadata_repository(&mut self) -> Self::MetadataRepository<'_>;

    fn coffee_repository(&mut self) -> Self::CoffeeRepository<'_>;

    async fn commit(self) -> Result<(), Report>;
}

pub trait UnitOfWorkFactory {
    type UnitOfWork: UnitOfWork;

    async fn transaction(&self) -> Result<Self::UnitOfWork, Report>;
}

pub trait MetadataRepository {
    async fn get_application_id(&self) -> Result<u32, Report>;

    async fn set_application_id(&self, id: u32) -> Result<(), Report>;

    async fn get_user_version(&self) -> Result<u32, Report>;

    async fn set_user_version(&self, id: u32) -> Result<(), Report>;
}

pub trait CoffeeRepository {
    async fn insert(&self, coffee: &Coffee) -> Result<(), Report>;

    async fn update(&self, coffee: &Coffee) -> Result<(), Report>;

    async fn find_by_id(&self, id: &CoffeeId) -> Result<Option<Coffee>, Report>;

    async fn find_all(&self) -> Result<Vec<Coffee>, Report>;
}
