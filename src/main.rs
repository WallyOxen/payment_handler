use std::{collections::HashMap, error::Error, fs::File, io::BufReader};

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

#[derive(Debug)]
struct UserAccount {
    client: u16,
    available: f64,
    held: f64,
    total: f64,
    locked: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("test_input.csv")?;
    let reader = BufReader::new(file);

    let mut accounts: HashMap<u16, UserAccount> = HashMap::new();
    let mut transactions: HashMap<u32, Transaction> = HashMap::new();

    let mut csv_reader = ReaderBuilder::new().trim(Trim::All).from_reader(reader);

    for result in csv_reader.deserialize() {
        let transaction: Transaction = result?;

        let account = accounts.entry(transaction.client).or_insert(UserAccount {
            client: transaction.client,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        });

        if account.locked {
            println!(
                "Account {} is locked! Transaction {} cancelled",
                account.client, transaction.transaction_id
            );
            continue;
        }

        match transaction.transaction_type {
            TransactionType::Chargeback => {
                if let Some(disputed_transaction) = transactions.get(&transaction.transaction_id) {
                    if let Some(disputed_amount) = disputed_transaction.amount {
                        account.held -= disputed_amount;
                        account.total -= disputed_amount;

                        if account.total != account.available - account.held {
                            println!(
                                "Something went wrong with this account/chargback! {}",
                                transaction.transaction_id
                            );
                        }
                    } else {
                        println!(
                            "Failed to find amount for disputed transaction {}",
                            transaction.transaction_id
                        );
                    }
                } else {
                    println!(
                        "Failed to find disputed transaction {}",
                        transaction.transaction_id
                    );
                }
            }
            TransactionType::Deposit => {
                if let Some(amount) = transaction.amount {
                    account.available += amount;
                    account.total += amount;

                    if account.total != account.available - account.held {
                        println!(
                            "Something went wrong with this account/deposit! {}",
                            transaction.transaction_id
                        );
                    }

                    transactions.insert(transaction.transaction_id, transaction);
                } else {
                    println!(
                        "Failed to find amount for deposit {}",
                        transaction.transaction_id
                    );
                }
            }
            TransactionType::Dispute => {
                if let Some(disputed_transaction) = transactions.get(&transaction.transaction_id) {
                    if let Some(disputed_amount) = disputed_transaction.amount {
                        account.available -= disputed_amount;
                        account.held += disputed_amount;

                        if account.total != account.available - account.held {
                            println!(
                                "Something went wrong with this account/dispute! {}",
                                transaction.transaction_id
                            );
                        }
                    } else {
                        println!(
                            "Failed to find amount for disputed transaction {}",
                            transaction.transaction_id
                        );
                    }
                } else {
                    println!(
                        "Failed to find disputed transaction {}",
                        transaction.transaction_id
                    );
                }
            }
            TransactionType::Resolve => {
                if let Some(disputed_transaction) = transactions.get(&transaction.transaction_id) {
                    if let Some(disputed_amount) = disputed_transaction.amount {
                        account.available += disputed_amount;
                        account.held -= disputed_amount;

                        if account.total != account.available - account.held {
                            println!(
                                "Something went wrong with this account/resolve! {}",
                                transaction.transaction_id
                            );
                        }
                    }
                    println!(
                        "Failed to find amount for disputed transaction {}",
                        transaction.transaction_id
                    );
                } else {
                    println!(
                        "Failed to find disputed transaction {}",
                        transaction.transaction_id
                    );
                }
            }
            TransactionType::Withdrawal => {
                if let Some(amount) = transaction.amount {
                    if account.available < amount {
                        println!(
                            "Account {} has insufficient funds for transaction {}",
                            account.client, transaction.transaction_id
                        );
                        continue;
                    }
                    account.available -= amount;
                    account.total -= amount;

                    if account.total != account.available - account.held {
                        println!(
                            "Something went wrong with this account/withdrawal! {}",
                            transaction.transaction_id
                        );
                    }

                    transactions.insert(transaction.transaction_id, transaction);
                } else {
                    println!(
                        "Failed to get amount for transaction {}",
                        transaction.transaction_id
                    );
                }
            }
        };
    }

    for account in accounts.values() {
        println!("{:#?}", account);
    }
    Ok(())
}
