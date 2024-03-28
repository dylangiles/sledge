use sea_orm::entity::prelude::*;
use sea_orm::{DeriveActiveEnum, DeriveEntityModel, EnumIter};
use tabled::Tabled;

#[derive(EnumIter, DeriveActiveEnum, Debug, Clone, PartialEq, Eq)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum AccountType {
    #[sea_orm(num_value = 1)]
    Asset,

    #[sea_orm(num_value = 2)]
    Liability,

    #[sea_orm(num_value = 3)]
    Equity,

    #[sea_orm(num_value = 4)]
    Revenue,

    #[sea_orm(num_value = 5)]
    Expense,
}

impl AccountType {
    pub fn friendly_name(&self) -> &'static str {
        match self {
            AccountType::Asset => "Asset",
            AccountType::Liability => "Liability",
            AccountType::Equity => "Equity",
            AccountType::Revenue => "Revenue",
            AccountType::Expense => "Expense",
        }
    }
}

impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.friendly_name())
    }
}

impl From<&str> for AccountType {
    fn from(value: &str) -> Self {
        match value {
            "asset" | "a" => AccountType::Asset,
            "liability" | "l" => AccountType::Liability,
            "equity" | "o" => AccountType::Equity,
            "revenue" | "r" => AccountType::Revenue,
            "expense" | "e" => AccountType::Expense,
            u => panic!("Unknown account type: {u}"),
        }
    }
}

impl From<String> for AccountType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "asset" | "a" => AccountType::Asset,
            "liability" | "l" => AccountType::Liability,
            "equity" | "o" => AccountType::Equity,
            "revenue" | "r" => AccountType::Revenue,
            "expense" | "e" => AccountType::Expense,
            u => panic!("Unknown account type: {u}"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Tabled)]
#[sea_orm(table_name = "accounts")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[tabled(rename = "ID")]
    pub id: i32,

    #[tabled(rename = "Account Code")]
    pub code: String,

    #[tabled(rename = "Account Name")]
    pub name: String,

    #[tabled(rename = "Account Type")]
    pub account_type: AccountType,

    #[tabled(rename = "System?")]
    pub system: bool,

    #[tabled(rename = "Postable?")]
    pub postable: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
