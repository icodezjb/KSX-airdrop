use substrate_subxt::balances::{TransferCallExt, TransferEventExt};
use substrate_subxt::system::AccountStoreExt;

use chainx_runtime::AccountId;

use crate::utils::{build_client, parse_account, Sr25519Signer};

/// Balances
#[derive(structopt::StructOpt, Debug)]
pub enum Balances {
    /// Transfer some balances from signer to another account.
    #[structopt(name = "transfer")]
    Transfer {
        /// receiver
        #[structopt(index = 1, long, parse(try_from_str = parse_account))]
        dest: AccountId,
        /// amount
        #[structopt(index = 2)]
        value: u128,
    },
    AccountInfo {
        #[structopt(index = 1, long, parse(try_from_str = parse_account))]
        who: AccountId,
    },
}

impl Balances {
    pub async fn run(
        self,
        url: String,
        signer: Sr25519Signer,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let client = build_client(url).await?;

        match self {
            Balances::AccountInfo { who } => {
                println!(
                    "AccountInfo of {:?}: {:#?}",
                    who,
                    client.account(&who, None).await?
                );
            }
            Balances::Transfer { dest, value } => {
                let result = client
                    .transfer_and_watch(&signer, &dest.into(), value)
                    .await?;
                if let Some(event) = result.transfer()? {
                    println!("Balance transfer success: value: {:?}", event.amount);
                } else {
                    println!("Failed to find Balances::Transfer Event");
                }
            }
        }

        Ok(())
    }
}