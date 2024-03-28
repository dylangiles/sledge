use entity::account::{AccountType, ActiveModel as AccountActiveModel};
use sea_orm_migration::{
    prelude::*,
    sea_orm::{ActiveModelTrait, Set},
};

use crate::iden::Accounts;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Accounts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Accounts::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Accounts::Code).string().not_null())
                    .col(ColumnDef::new(Accounts::Name).string().not_null())
                    .col(ColumnDef::new(Accounts::AccountType).small_integer())
                    .col(ColumnDef::new(Accounts::System).boolean().not_null())
                    .col(ColumnDef::new(Accounts::Postable).boolean().not_null())
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        AccountActiveModel {
            code: Set("0000".into()),
            name: Set("Assets".into()),
            account_type: Set(AccountType::Asset),
            system: Set(true),
            postable: Set(false),
            ..Default::default()
        }
        .insert(db)
        .await?;

        AccountActiveModel {
            code: Set("0000".into()),
            name: Set("Liabilities".into()),
            account_type: Set(AccountType::Liability),
            system: Set(true),
            postable: Set(false),
            ..Default::default()
        }
        .insert(db)
        .await?;

        AccountActiveModel {
            code: Set("0000".into()),
            name: Set("Equity".into()),
            account_type: Set(AccountType::Equity),
            system: Set(true),
            postable: Set(false),
            ..Default::default()
        }
        .insert(db)
        .await?;

        AccountActiveModel {
            code: Set("1000".into()),
            name: Set("Opening Equity".into()),
            account_type: Set(AccountType::Equity),
            system: Set(true),
            postable: Set(true),
            ..Default::default()
        }
        .insert(db)
        .await?;

        AccountActiveModel {
            code: Set("0000".into()),
            name: Set("Revenue".into()),
            account_type: Set(AccountType::Revenue),
            system: Set(true),
            postable: Set(false),
            ..Default::default()
        }
        .insert(db)
        .await?;

        AccountActiveModel {
            code: Set("0000".into()),
            name: Set("Expenses".into()),
            account_type: Set(AccountType::Expense),
            system: Set(true),
            postable: Set(false),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Accounts::Table).to_owned())
            .await
    }
}
