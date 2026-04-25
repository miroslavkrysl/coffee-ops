use crate::repository::coffee::TursoCoffeeRepository;
use crate::repository::metadata::TursoMetadataRepository;
use coffee_ops_core::port::db::{UnitOfWork, UnitOfWorkFactory};
use rootcause::prelude::*;
use turso::{Connection, Database};

#[derive(Debug)]
pub struct TursoUnitOfWorkFactory {
    database: Database,
}

impl TursoUnitOfWorkFactory {
    pub fn new(database: Database) -> Self {
        Self { database }
    }
}

impl UnitOfWorkFactory for TursoUnitOfWorkFactory {
    type UnitOfWork = TursoUnitOfWork;

    async fn transaction(&self) -> Result<TursoUnitOfWork, Report> {
        let connection = self.database.connect()?;
        Ok(TursoUnitOfWork::new(connection).await?)
    }
}

#[derive(Debug)]
pub struct TursoUnitOfWork {
    transaction: Transaction,
}

impl TursoUnitOfWork {
    pub async fn new(connection: Connection) -> Result<Self, Report> {
        let transaction = Transaction::begin(connection).await?;

        Ok(Self { transaction })
    }
}

impl UnitOfWork for TursoUnitOfWork {
    type MetadataRepository<'a> = TursoMetadataRepository<'a>;

    type CoffeeRepository<'a> = TursoCoffeeRepository<'a>;

    fn metadata_repository(&mut self) -> Self::MetadataRepository<'_> {
        todo!()
    }

    fn coffee_repository(&mut self) -> Self::CoffeeRepository<'_> {
        todo!()
    }

    async fn commit(self) -> Result<(), Report> {
        self.transaction.commit().await?;
        Ok(())
    }
}

#[derive(Debug)]
struct Transaction {
    connection: Connection,
    in_progress: bool,
}

impl Transaction {
    pub async fn begin(connection: Connection) -> Result<Self, Report> {
        connection.execute("BEGIN", ()).await?;

        Ok(Self {
            connection,
            in_progress: true,
        })
    }

    pub fn connection(&self) -> &Connection {
        &self.connection
    }

    pub async fn commit(mut self) -> Result<(), Report> {
        self.connection.execute("COMMIT", ()).await?;
        self.in_progress = false;
        Ok(())
    }

    pub async fn rollback(mut self) -> Result<(), Report> {
        self.connection.execute("ROLLBACK", ()).await?;
        self.in_progress = false;
        Ok(())
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        if self.in_progress {
            let connection = self.connection.clone();
            tokio::spawn(async move {
                if let Err(e) = connection.execute("ROLLBACK", ()).await {
                    tracing::error!("failed to rollback transaction: {e}");
                }
            });
        }
    }
}
