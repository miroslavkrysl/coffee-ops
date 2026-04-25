use rootcause::Report;
use turso::transaction::Transaction;
use coffee_ops_core::domain::coffee::{Coffee, CoffeeId};
use coffee_ops_core::port::db::CoffeeRepository;

pub mod coffee;
pub mod metadata;