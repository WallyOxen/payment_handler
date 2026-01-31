use std::{error::Error, io};

use csv::{ReaderBuilder, Trim};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Transaction {
    #[serde(rename = "type")]
    transaction_type: TransactionType,
    client: u16,
    #[serde(rename = "tx")]
    transaction_id: u32,
    amount: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum TransactionType {
    Chargeback,
    Deposit,
    Dispute,
    Resolve,
    Withdrawal,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
        .trim(Trim::All)
        .from_reader(io::stdin());

    for result in reader.deserialize() {
        let transaction: Transaction = result?;
        println!("{:#?}", transaction);
    }
    Ok(())
}
