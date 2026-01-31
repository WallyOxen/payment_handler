use std::{
    collections::HashMap,
    env::args,
    error::Error,
    fs::File,
    io::{self, BufReader},
    process,
};

use csv::{ReaderBuilder, Trim, Writer};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Transaction {
    #[serde(rename = "type")]
    transaction_type: TransactionType,
    client: u16,
    #[serde(rename = "tx")]
    transaction_id: u32,
    amount: Option<Decimal>,
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

#[derive(Debug, Serialize)]
struct UserAccount {
    client: u16,
    available: Decimal,
    held: Decimal,
    total: Decimal,
    locked: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().collect();

    if args.len() > 2 || args.len() == 1 {
        eprintln!("Expected only 1 argument of input file name");
        process::exit(1);
    }

    // Safe to unwrap index 1 as above ensures there is a value
    let file = File::open(args.get(1).unwrap())?;
    let reader = BufReader::new(file);

    let mut accounts: HashMap<u16, UserAccount> = HashMap::new();
    let mut transactions: HashMap<u32, Transaction> = HashMap::new();

    let mut csv_reader = ReaderBuilder::new().trim(Trim::All).from_reader(reader);

    for result in csv_reader.deserialize() {
        let transaction: Transaction = result?;

        let account = accounts.entry(transaction.client).or_insert(UserAccount {
            client: transaction.client,
            available: Decimal::new(0, 4),
            held: Decimal::new(0, 4),
            total: Decimal::new(0, 4),
            locked: false,
        });

        if account.locked {
            eprintln!(
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
                        account.locked = true;

                        if account.total != account.available + account.held {
                            eprintln!(
                                "Something went wrong with this account/chargback! {}",
                                transaction.transaction_id
                            );
                        }
                    } else {
                        eprintln!(
                            "Failed to find amount for disputed transaction {}",
                            transaction.transaction_id
                        );
                    }
                }
            }
            TransactionType::Deposit => {
                if let Some(amount) = transaction.amount {
                    account.available += amount;
                    account.total += amount;

                    if account.total != account.available + account.held {
                        eprintln!(
                            "Something went wrong with this account/deposit! {}",
                            transaction.transaction_id
                        );
                    }

                    transactions.insert(transaction.transaction_id, transaction);
                } else {
                    eprintln!(
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

                        if account.total != account.available + account.held {
                            eprintln!(
                                "Something went wrong with this account/dispute! {}",
                                transaction.transaction_id
                            );
                        }
                    } else {
                        eprintln!(
                            "Failed to find amount for disputed transaction {}",
                            transaction.transaction_id
                        );
                    }
                }
            }
            TransactionType::Resolve => {
                if let Some(disputed_transaction) = transactions.get(&transaction.transaction_id) {
                    if let Some(disputed_amount) = disputed_transaction.amount {
                        account.available += disputed_amount;
                        account.held -= disputed_amount;

                        if account.total != account.available + account.held {
                            eprintln!(
                                "Something went wrong with this account/resolve! {}",
                                transaction.transaction_id
                            );
                        }
                    } else {
                        eprintln!(
                            "Failed to find amount for disputed transaction {}",
                            transaction.transaction_id
                        );
                    }
                }
            }
            TransactionType::Withdrawal => {
                if let Some(amount) = transaction.amount {
                    if account.available < amount {
                        eprintln!(
                            "Account {} has insufficient funds for transaction {}",
                            account.client, transaction.transaction_id
                        );
                        continue;
                    }
                    account.available -= amount;
                    account.total -= amount;

                    if account.total != account.available + account.held {
                        eprintln!(
                            "Something went wrong with this account/withdrawal! {}",
                            transaction.transaction_id
                        );
                    }

                    transactions.insert(transaction.transaction_id, transaction);
                } else {
                    eprintln!(
                        "Failed to get amount for transaction {}",
                        transaction.transaction_id
                    );
                }
            }
        };
    }

    let mut csv_writer = Writer::from_writer(io::stdout());

    for account in accounts.values() {
        csv_writer.serialize(account)?;
    }

    csv_writer.flush()?;
    Ok(())
}
