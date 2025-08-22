#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20250714_194711_beans;
mod m20250714_194918_espressos;
mod m20250822_124544_add_grind_size_to_espresso;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20250714_194711_beans::Migration),
            Box::new(m20250714_194918_espressos::Migration),
            Box::new(m20250822_124544_add_grind_size_to_espresso::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
