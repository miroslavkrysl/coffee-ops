use std::path::Path;
use std::sync::LazyLock;
use turso::Database;
use coffee_ops_core::port::db::UnitOfWorkFactory;

// #[derive(Debug, Clone, Copy)]
// pub struct MigratorConfig {
//     pub application_id: u32,
// }
//
// pub struct Migrator {
//     uow_factory: UnitOfWorkFactory<UnitOfWork=()>,
//     config: MigratorConfig,
// }
//
// impl Migrator {
//
//     pub fn new(database: Database, config: MigratorConfig) -> Self {
//         Self { database, config }
//     }
// }
//
//
// static MIGRATIONS: LazyLock<Vec<Migration>> = LazyLock::new(|| {
//     vec![
//         Migration { sql: "zero".into() },  // key 0
//         Migration { value: "one".into() },   // key 1
//         Migration { value: "two".into() },   // key 2
//     ]
// });
//
// static SCHEMA: Migration = Migration {
//
// }
//
// struct Migration {
//     sql: String,
// }
//
// impl Migration {
//
//     pub fn from_file(file_name: &Path) -> Self {
//         let sql = ""
//     }
// }