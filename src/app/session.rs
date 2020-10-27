use anyhow::Result;
use subxt::session::ValidatorsStoreExt;

use crate::{runtime::ChainXSigner, utils::build_client};

/// Session
#[derive(structopt::StructOpt, Debug)]
pub enum Session {
    #[structopt(name = "set-keys")]
    SetKeys {
        #[structopt(index = 1, long)]
        keys: String,
    },
    #[structopt(name = "validators")]
    Validators,
}

impl Session {
    pub async fn run(self, url: String, _signer: ChainXSigner) -> Result<()> {
        let client = build_client(url).await?;

        match self {
            Self::Validators => {
                println!("{:#?}", client.validators(None).await?);
            }
            Self::SetKeys { keys } => {
                let _ = keys;
                todo!()
                // let result = client.set_keys_and_watch(&signer, &call).await?;
                // println!("{:#?}", result);
            }
        }

        Ok(())
    }
}