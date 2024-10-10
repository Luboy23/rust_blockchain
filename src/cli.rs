use std::process::exit;

use bitcoincash_addr::Address;
use clap::{arg, Command};

use crate::blockchain::Blockchain;
use crate::errors::Result;
use crate::transaction::Transaction;
use crate::utxoset::UTXOSet;
use crate::wallet::Wallets;

pub struct Cli {
}

impl Cli {
    pub fn new() -> Result<Cli> {
        Ok(Cli {})
    }

    pub fn run(&mut self) -> Result<()> {
        let matches = Command::new("blockchain-rust-demo")
            .version("0.1")
            .author("lllu238744@gmail.com")
            .about("blockchain in rust: a simple blockchain for learning")

            .subcommand(Command::new("printchain").about("print all the chain blocks"))

            .subcommand(Command::new("createwallet").about("create a new wallet"))

            .subcommand(Command::new("listaddresses").about("list all addresses"))

            .subcommand(Command::new("getbalance")
            .about("get balance in the blockchain")
            .arg(arg!(<ADDRESS>"'The Address it get balance for'")))

            .subcommand(Command::new("create")
                .about("Create new blockchain")
                .arg(arg!(<ADDRESS>"'the address to send genesis block reward to'")))

            .subcommand(
                Command::new("send")
                    .about("send in the blockchain")
                    .arg(arg!(<FROM>"'Source wallet address'"))
                    .arg(arg!(<TO>"'Destination wallet address'"))
                    .arg(arg!(<AMOUNT>"'Amount to transfer'")))
            .subcommand(Command::new("reindex").about("reindex UTXO"))
            .get_matches();

        if let Some(ref matches) = matches.subcommand_matches("create") {
            if let Some(address) = matches.get_one::<String>("ADDRESS") {
                let address = String::from(address);
                let bc = Blockchain::create_blockchain(address.clone())?;
                let utxo_set = UTXOSet {blockchain: bc};
                utxo_set.reindex()?;
                println!("create blockchain");
            }
        }

        if let Some(ref matches) = matches.subcommand_matches("getbalance") {
            if let Some(address) = matches.get_one::<String>("ADDRESS") {
                let pub_key_hash = Address::decode(address).unwrap().body;
                let bc = Blockchain::new()?;
                // let utxos = bc.find_utxo(&pub_key_hash);
                let utxo_set = UTXOSet {blockchain: bc};
                let utxos = utxo_set.find_utxo(&pub_key_hash)?;

                let mut balance: i32 = 0;
                for out in utxos.outputs {
                    balance += out.value;
                }
                println!("Balance of '{}'; {}", address, balance)
            }
        }

        if let Some(ref matches) = matches.subcommand_matches("send") {
            let from = if let Some(address) = matches.get_one::<String>("FROM") {
                address
            } else {
                println!("from not supply!: usage");
                exit(1)
            };

            let to = if let Some(address) = matches.get_one::<String>("TO") {
                address
            } else {
                println!("from not supply!: usage");
                exit(1)
            };

            let amount: i32 = if let Some(amount) = matches.get_one::<String>("AMOUNT") {
                amount.parse()?
            } else {
                println!("from not supply!: usage");
                exit(1)
            };

            let  bc = Blockchain::new()?;
            let mut utxo_set = UTXOSet {blockchain:bc};
            let tx = Transaction::new_utxo(from, to, amount, &utxo_set)?;
            let cbtx = Transaction::new_coinbase(from.to_string(), String::from("Reawad!"))?;
            let new_block = utxo_set.blockchain.add_block(vec![cbtx, tx])?;

            utxo_set.update(&new_block)?;
            println!("success!");
        }


        if let Some(_) = matches.subcommand_matches("printchain") {
            let bc = Blockchain::new()?;
            for b in &mut bc.iter() {
                println!("block: {:#?}", b);
            }
        }

        if let Some(_) = matches.subcommand_matches("createwallet") {
            let mut ws = Wallets::new()?;
            let address = ws.create_wallet();
            ws.save_all()?;
            println!("success: address {}", address);
        }

        if let Some(_) = matches.subcommand_matches("listaddresses") {
            let  ws = Wallets::new()?;
            let addresses = ws.get_all_address();
            println!("addresses:");
            for ad in addresses {
                println!("{}", ad);
            }
        }

        if let Some(_) = matches.subcommand_matches("reindex") {
            let bc = Blockchain::new()?;
            let utxo_set = UTXOSet {blockchain: bc};
            utxo_set.reindex()?;
            let count = utxo_set.count_transaction()?;
            println!("Done! There are {} transactions in the UTXO set.", count);
        }
    

        Ok(())
    }

}