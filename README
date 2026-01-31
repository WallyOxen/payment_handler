# Payment Handler
This is a simple example payment handler. It handles deposits, withdrawals, disputes, resolves and chargebacks.

All currency is 4 decimal place precision, handled by the Rust_Decimal crate.

Included are two simple test files used during development (test_input.csv and test_input2.csv).


### Future Additions / Stretch Goals
- Adding a trait (with a default to in memory store as exists currently) to handle how accounts and transactions are stored and read to add modularity and simplify any future use cases/migrations.

- Adding clap crate to bolster the cli interface and polish the experience

- Updating the transaction struct to be more developer friendly
    - Creating transaction types with only known information (i.e. Disputes, resolves and chargebacks do not contain amounts)

- Split out logic code from the match on transaction_type into individual functions in order to provide a pathway to adding unit tests