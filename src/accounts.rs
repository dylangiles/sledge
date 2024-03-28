use entity::account::{
    self, AccountType, ActiveModel as AccountActiveModel, Entity as AccountEntity,
    Model as AccountModel,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, Order, QueryFilter,
    QueryOrder, QuerySelect, Set,
};
use tabled::settings::Style;

use crate::db::get_db;

pub(super) async fn add_account<A>(
    name: String,
    account_type: A,
    code: Option<String>,
) -> Result<AccountModel, DbErr>
where
    A: Into<AccountType>,
{
    let db = get_db().await?;
    let account_type = account_type.into();
    let item = AccountActiveModel {
        code: Set(if let Some(code) = code {
            code
        } else {
            get_next_account_code(account_type.clone(), Some(db.clone())).await?
        }),
        name: Set(name),
        account_type: Set(account_type),
        ..Default::default()
    };

    let item = item.insert(&db).await?;
    Ok(item)
}

pub(super) async fn get_next_account_code(
    account_type: AccountType,
    db: Option<DatabaseConnection>,
) -> Result<String, DbErr> {
    let db = if let Some(db) = db {
        db
    } else {
        get_db().await?
    };

    let record = AccountEntity::find()
        .filter(account::Column::AccountType.eq(account_type))
        .order_by(account::Column::Code, Order::Desc)
        .one(&db)
        .await?;

    let result = match record {
        Some(rec) => {
            let mut int_current = match rec.code.parse::<i32>() {
                Ok(v) => v,
                Err(e) => panic!("{e}"),
            };

            if int_current == 9999 {
                panic!("out of account space")
            };

            int_current += 1;
            format!("{int_current:0>4}")
        }

        None => "0000".into(),
    };

    Ok(result)
}

pub(crate) async fn get_accounts() -> Result<Vec<AccountModel>, DbErr> {
    let db = get_db().await?;
    let records = AccountEntity::find().all(&db).await?;
    Ok(records)
}

pub(crate) async fn list_accounts() -> Result<(), DbErr> {
    use tabled::Table;

    println!(
        "{}",
        Table::new(get_accounts().await?)
            .with(Style::modern())
            .to_string()
    );

    Ok(())
}
