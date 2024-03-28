use std::error::Error;

use clap::{Args, Parser, Subcommand};
use entity::account::AccountType;

use crate::accounts;

#[derive(Parser)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Subcommands,
}

#[derive(Subcommand)]
pub(crate) enum Subcommands {
    /// List accounts
    #[command(subcommand)]
    Accounts(AccountsCommand),
}

#[derive(Subcommand)]
pub(crate) enum AccountsCommand {
    /// Add an account to the ledger
    Add(AddAccountArgs),

    #[command(subcommand)]
    List(ListAccountsCommand),
}

#[derive(Debug, Args)]
struct AddAccountArgs {
    /// The name of the new account
    #[arg(short = 'n', long = "name")]
    name: String,

    /// The account type of the new account
    ///
    /// Must be one of the following values: asset, liability, equity, revenue, expense (a, l, o, r, e)
    #[arg(short = 't', long = "type")]
    account_type: String,

    /// The account code. If omitted, one will be generated for you.
    #[arg(short, long)]
    code: Option<String>,
}

#[derive(Subcommand)]
pub(crate) enum ListAccountsCommand {
    /// List all accounts in the ledger
    All,
}

pub(super) async fn cli() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match cli.command {
        Subcommands::Accounts(acc) => match acc {
            AccountsCommand::Add(args) => {
                let account =
                    accounts::add_account(args.name, args.account_type, args.code).await?;
                println!("Created account {}", account.name)
            }
            AccountsCommand::List(list) => match list {
                ListAccountsCommand::All => accounts::list_accounts().await?,
            },
        },
    }

    Ok(())
}
