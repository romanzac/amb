#![allow(unused)]

use std::str::FromStr;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use clap::{Command};
use settings::Settings;
use ethers::{signers::LocalWallet, providers::{Provider, Http}};
use ethers::contract::abigen;
use ethers::providers::Middleware;
use ethers::middleware::SignerMiddleware;
use ethers::signers::{Signer};
use ethers::types::{BlockId, BlockNumber, H160, U256};

mod settings;

abigen!(
    // Counter ABI
    Counter, "../build/contracts/CounterN1.json",
    event_derives(serde::Deserialize, serde::Serialize);
);


#[macro_use]
extern crate serde_derive;

fn cli() -> Command<'static> {
    Command::new("relayer")
        .about("Trusted Relayer watches counter N1 and increments N2")
        .subcommand(
            Command::new("increment")
                .about("Increments N1 by one using DAPP account")
        )
}

pub async fn run(cmd: &str, settings: &Settings) {

    let dapp_wallet = settings.r.dapp_mnemonic.parse::<LocalWallet>().unwrap();
    let dapp_wallet = dapp_wallet.with_chain_id(settings.r.n1_chain_id.parse::<u64>().unwrap());
    let amb_n1_wallet = settings.r.amb_mnemonic.parse::<LocalWallet>().unwrap();
    let amb_n2_wallet = settings.r.amb_mnemonic.parse::<LocalWallet>().unwrap();
    let amb_n1_wallet = amb_n1_wallet.with_chain_id(settings.r.n1_chain_id.parse::<u64>().unwrap());
    let amb_n2_wallet = amb_n2_wallet.with_chain_id(settings.r.n2_chain_id.parse::<u64>().unwrap());

    // Connect to the networks
    let provider_n1 = Provider::<Http>::try_from(
        settings.r.n1_rpc_url.clone()
    ).expect("could not instantiate RPC Provider N1");

    let provider_n2 = Provider::<Http>::try_from(
        settings.r.n2_rpc_url.clone()
    ).expect("could not instantiate RPC Provider N2");

    let last_block_n1 = provider_n1.get_block_number().await.unwrap();
    let last_block_n1 = BlockId::from(last_block_n1);

    let last_block_n2 = provider_n2.get_block_number().await.unwrap();
    let last_block_n2 = BlockId::from(last_block_n2);


    let dapp_address = H160::from_str(&settings.r.dapp_address).unwrap();
    let amb_address = H160::from_str(&settings.r.amb_address).unwrap();

    let dapp_init_nonce_n1 = provider_n1.get_transaction_count(dapp_address, Some(last_block_n1)).await.unwrap();
    let amb_init_nonce_n2 = provider_n2.get_transaction_count(amb_address, Some(last_block_n2)).await.unwrap();

    // Instantiate the clients with their wallets
    let dapp_client_n1 = SignerMiddleware::new(provider_n1.clone(), dapp_wallet);
    let dapp_client_n1 = Arc::new(dapp_client_n1);

    let amb_client_n1 = SignerMiddleware::new(provider_n1.clone(), amb_n1_wallet);
    let amb_client_n1 = Arc::new(amb_client_n1);
    let amb_client_n2 = SignerMiddleware::new(provider_n2.clone(), amb_n2_wallet);
    let amb_client_n2 = Arc::new(amb_client_n2);

    let counter_n1_addr = H160::from_str(&settings.r.counter_n1).unwrap();
    let counter_n2_addr = H160::from_str(&settings.r.counter_n2).unwrap();

    let counter_n1_sc_dapp = Counter::new(counter_n1_addr, dapp_client_n1.clone());
    let counter_n1_sc_amb = Counter::new(counter_n1_addr, amb_client_n1.clone());
    let counter_n2_sc_amb = Counter::new(counter_n2_addr, amb_client_n2.clone());

    let gas_price_n1 = U256::from(settings.r.n1_gas_price.parse::<u64>().unwrap());
    let gas_limit_n1 = U256::from(settings.r.n1_gas_limit.parse::<u64>().unwrap());
    let gas_price_n2 = U256::from(settings.r.n2_gas_price.parse::<u64>().unwrap());
    let gas_limit_n2 = U256::from(settings.r.n2_gas_limit.parse::<u64>().unwrap());


    match cmd {
        "increment" => {
            let mut nonce = dapp_init_nonce_n1;
            println!("Going to increment Counter N1...\n");
            let mut increment_n1_call = counter_n1_sc_dapp.increment(dapp_address, U256::zero())
                .from(dapp_address)
                .gas_price(gas_price_n1)
                .gas(gas_limit_n1);

            increment_n1_call.tx.set_nonce(nonce);

            match increment_n1_call.send().await {
                Ok(tx) => {
                    println!("Counter N1 has been incremented tx: {:?} \n", tx);
                }
                Err(e) => {
                    println!("Error during increment Counter N1: {:?}", e);
                }
            };

            nonce = nonce + 1;
            sleep(Duration::from_secs(10));

            println!("Going to send Counter N1 value to N2...\n");
            let mut send_from_n1_call = counter_n1_sc_dapp.send()
                .from(dapp_address)
                .gas_price(gas_price_n1)
                .gas(gas_limit_n1);

            send_from_n1_call.tx.set_nonce(nonce);
            send_from_n1_call.tx.set_value(gas_limit_n2);

            match send_from_n1_call.send().await {
                Ok(tx) => {
                    println!("Counter N1 has sent value to N2 tx: {:?} \n", tx);
                }
                Err(e) => {
                    println!("Error during sending value of Counter N1 over to N2: {:?}", e);
                }
            };
        },

        "relayer" => {
            let mut nonce = amb_init_nonce_n2;
            let (mut orig_value_to_send, mut new_value_to_send) = (U256::zero(), U256::zero());

            loop {
                new_value_to_send = counter_n1_sc_amb.get_target_value(counter_n2_addr).call().await.unwrap();

                if new_value_to_send > orig_value_to_send {
                    println!("Going to update Counter N2 to value {:?} from N1...\n", new_value_to_send);
                    let mut increment_n2_call = counter_n2_sc_amb.increment(counter_n2_addr, new_value_to_send)
                        .from(amb_address)
                        .gas_price(gas_price_n2)
                        .gas(gas_limit_n2);

                    increment_n2_call.tx.set_nonce(nonce);

                    match increment_n2_call.send().await {
                        Ok(tx) => {
                            println!("Counter N2 has been updated tx: {:?} \n", tx);
                            orig_value_to_send = new_value_to_send;
                            nonce = nonce + 1;
                        }
                        Err(e) => {
                            println!("Error during Counter N2 update: {:?}", e);
                            break
                        }
                    };
                }
                sleep(Duration::from_secs(10));
            }
        },
        _ => {}
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let settings = Settings::new().unwrap();

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("increment", _)) => run("increment", &settings).await,
        Some((name, _)) => {
            unreachable!("Unsupported subcommand `{}`", name)
        },
        None => run("relayer", &settings).await,
    }

    Ok(())
}
