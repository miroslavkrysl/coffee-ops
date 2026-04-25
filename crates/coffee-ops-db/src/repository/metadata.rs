use coffee_ops_core::port::db::MetadataRepository;
use rootcause::Report;
use turso::transaction::Transaction;

pub struct TursoMetadataRepository<'tx> {
    transaction: &'tx Transaction<'tx>,
}

impl MetadataRepository for TursoMetadataRepository<'_> {
    async fn get_application_id(&self) -> Result<u32, Report> {
        todo!()
    }

    async fn set_application_id(&self, id: u32) -> Result<(), Report> {
        todo!()
    }

    async fn get_user_version(&self) -> Result<u32, Report> {
        todo!()
    }

    async fn set_user_version(&self, id: u32) -> Result<(), Report> {
        todo!()
    }
}
