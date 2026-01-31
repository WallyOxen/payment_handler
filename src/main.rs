use std::{error::Error, fs::File, io::BufReader};

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
    let file = File::open("test_input.csv")?;
    let reader = BufReader::new(file);

    let mut csv_reader = ReaderBuilder::new().trim(Trim::All).from_reader(reader);

    for result in csv_reader.deserialize() {
        let transaction: Transaction = result?;
        println!("{:#?}", transaction);
    }
    Ok(())
}
