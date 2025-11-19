# RustLedger

## Assumptions
- All amounts associated with deposit/withdrawal transactions should be positive values
- Only a deposit transaction can be disputed
- A dispute will only be processed if the disputed amount is less than or equal to their available funds
- If an account is frozen, no further transactions will be processed for that account

## Error Handling
- If any row in the transactions data does not hold an appropriate type, that transaction will be skipped. E.g. a client id which should be a `u16` coming in as `abc`
- Likewise, a transaction with a negative amount will be ignored
- deposit/withdrawal transactions with empty amount fields will be ignored
- A dispute/resolve/chargeback with a non-existant transaction id for a client will be ignored

## Tests
Unit tests for key functions in `src/ledger.rs` have been included in the bottom of the same file under a separate module. 
A large range of functional test cases has also been included in `src/tests`.

Both lists of tests can be run by running `cargo test` from the root folder.
