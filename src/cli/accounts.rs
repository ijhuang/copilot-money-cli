use comfy_table::Cell;

use crate::client::CopilotClient;

use super::render::{TableRow, render_table, shorten_id_for_table};
use super::{AccountsCmd, Cli, OutputFormat};

pub(super) fn run_accounts(
    cli: &Cli,
    client: &CopilotClient,
    cmd: AccountsCmd,
) -> anyhow::Result<()> {
    match cmd {
        AccountsCmd::List(args) => {
            let mut items = client.list_accounts()?;
            if !args.all {
                items.retain(|account| {
                    !account.is_user_closed.unwrap_or(false)
                        && !account.is_user_hidden.unwrap_or(false)
                });
            }

            match cli.output {
                OutputFormat::Json => {
                    println!("{}", serde_json::to_string_pretty(&items)?);
                    Ok(())
                }
                OutputFormat::Table => {
                    let rows = items
                        .iter()
                        .map(|account| AccountRow {
                            id: account.id.clone(),
                            name: account.name.clone().unwrap_or_default(),
                            balance: super::value_to_money_string(account.balance.clone()),
                            latest_balance_update: super::value_to_string(
                                account.latest_balance_update.clone(),
                            ),
                            live: account.has_live_balance.unwrap_or(false).to_string(),
                        })
                        .collect();
                    render_table(cli, rows)
                }
            }
        }
    }
}

struct AccountRow {
    id: crate::types::AccountId,
    name: String,
    balance: String,
    latest_balance_update: String,
    live: String,
}

impl TableRow for AccountRow {
    const HEADERS: &'static [&'static str] =
        &["id", "name", "balance", "latest_balance_update", "live"];

    fn cells(&self) -> Vec<Cell> {
        vec![
            Cell::new(shorten_id_for_table(self.id.as_str())),
            Cell::new(&self.name),
            Cell::new(&self.balance),
            Cell::new(&self.latest_balance_update),
            Cell::new(&self.live),
        ]
    }
}
