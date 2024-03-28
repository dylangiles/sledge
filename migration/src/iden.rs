use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::DeriveIden;

#[derive(DeriveIden)]
pub(crate) enum Accounts {
    Table,
    Id,
    Code,
    Name,
    AccountType,
    System,
    Postable,
}
