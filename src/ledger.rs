use std::collections::HashMap;
use csv::{ReaderBuilder, Reader};
use crate::utils::{Account, Transaction, process_row};

pub fn get_transactions(file_path: &str) -> HashMap<u16, Account> {
    let mut rdr: Reader<std::fs::File> = ReaderBuilder::new().from_path(file_path).unwrap();
    let mut accounts: HashMap<u16, Account> = HashMap::new();
    
    for result in rdr.records() {
        match result {
            Ok(rcd) => {
                    let (tx_type, client_id, tx, amount) = match process_row(rcd) {
                    Ok(data) => data,
                    Err(_) => continue,
                };
                let mut transaction: Transaction = Transaction {
                    tx_type,
                    client_id,
                    tx,
                    amount,
                    disputed: false,
                };
                process_transaction(&mut transaction, &mut accounts);
            }
            Err(_) => continue,
        }
    }

    accounts
}

pub fn process_transaction(transaction: &mut Transaction, accounts: &mut HashMap<u16, Account>) {

    accounts.entry(transaction.client_id).or_insert_with(|| Account {
        available: 0.0,
        held: 0.0,
        locked: false,
        transactions: vec![],
    });
    let account: &mut Account = accounts.get_mut(&transaction.client_id).unwrap();
    if !account.locked {
        transaction_engine(transaction, account);
    }
}

fn transaction_engine(transaction: &mut Transaction, account: &mut Account) {
    match transaction.tx_type.as_str() {
        "deposit" => {
            if let Some(deposit_amount) = transaction.amount {
                account.available += deposit_amount;
                add_transaction_to_account(account, transaction);
            };
        },
        "withdrawal" => {
             if let Some(withdrawal_amount) = transaction.amount 
                && account.available >= withdrawal_amount {
                     account.available -= withdrawal_amount;
                     add_transaction_to_account(account, transaction);
                
            };
        },
        "dispute" => {
            let search_transaction: Option<&mut Transaction> = search_matching_deposit_transaction(transaction.tx, &mut account.transactions, false);
            if let Some(deposit_transaction) = search_transaction
                && deposit_transaction.amount.unwrap() <= account.available {
                    let amount_move_to_held: f32 = deposit_transaction.amount.unwrap();
                    account.available -= amount_move_to_held;
                    account.held += amount_move_to_held;
                    deposit_transaction.disputed = true;
                    add_transaction_to_account(account, transaction);
                }
        },
        "resolve" => {
            let search_transaction: Option<&mut Transaction> = search_matching_disputed_transaction(transaction.tx, &mut account.transactions);
            if let Some(deposit_transaction) = search_transaction {
                let amount_move_to_available: f32 = deposit_transaction.amount.unwrap();
                account.held -= amount_move_to_available;
                account.available += amount_move_to_available;
                deposit_transaction.disputed = false;
                add_transaction_to_account(account, transaction);
            }
        },
        "chargeback" => {
            let disputed_transaction: Option<&mut Transaction> = search_matching_disputed_transaction(transaction.tx, &mut account.transactions);
            if let Some(deposit_transaction) = disputed_transaction {
                account.held -= deposit_transaction.amount.unwrap();
                account.locked = true;
                add_transaction_to_account(account, transaction);
            }
        },
        _ => {},
    }
}

fn search_matching_deposit_transaction(tx: u32, account_transactions: &mut Vec<Transaction>, is_disputed: bool) -> Option<&mut Transaction> {
    for transaction in account_transactions.as_mut_slice() {
        if transaction.tx == tx && transaction.tx_type == "deposit" && transaction.amount.is_some() {
            if transaction.disputed == is_disputed {
                return Some(transaction);
            }
            break
        }
    }
    None
}

fn search_matching_disputed_transaction(tx: u32, account_transactions: &mut Vec<Transaction>) -> Option<&mut Transaction> {
    for transaction in account_transactions.iter() {
        if transaction.tx == tx && transaction.tx_type == "dispute" {
                return search_matching_deposit_transaction(tx, account_transactions, true);
        }
    }
    None
}

fn add_transaction_to_account(account: &mut Account, transaction: &Transaction) {
    account.transactions.push(
        transaction.clone()
    );
}

#[cfg(test)]
mod unittests {
    use super::*;

    #[test]
    fn test_process_transaction() {
        let transaction: &mut Transaction = &mut Transaction {
            tx_type: "deposit".to_string(),
            client_id: 1,
            tx: 1,
            amount: Some(100.0),
            disputed: false,
        };
        let accounts: &mut HashMap<u16, Account> = &mut HashMap::new();
        process_transaction(transaction, accounts);

        let account: &Account = accounts.get(&1).unwrap();
        assert_eq!(account.available, 100.0);
        assert_eq!(account.held, 0.0);
        assert_eq!(account.locked, false);
        assert_eq!(account.transactions.len(), 1);
    }

    #[test]
    fn test_process_transaction_locked() {
        let transactions: &mut Vec<Transaction> = &mut vec![
            Transaction {
                tx_type: "deposit".to_string(),
                client_id: 1,
                tx: 1,
                amount: Some(100.0),
                disputed: false,
            },
            Transaction {
                tx_type: "dispute".to_string(),
                client_id: 1,
                tx: 1,
                amount: None,
                disputed: false,
            },
            Transaction {
                tx_type: "chargeback".to_string(),
                client_id: 1,
                tx: 1,
                amount: None,
                disputed: false,
            },
        ];
        let accounts: &mut HashMap<u16, Account> = &mut HashMap::new();
        
        for transaction in transactions.iter_mut() {
            process_transaction(transaction, accounts);
        }

        let account: &Account = accounts.get(&1).unwrap();
        assert_eq!(account.available, 0.0);
        assert_eq!(account.held, 0.0);
        assert!(account.locked);
        assert_eq!(account.transactions.len(), 3);
    }

    #[test]
    fn test_transaction_engine_deposit() {
        let mut transaction: Transaction = Transaction {
            tx_type: "deposit".to_string(),
            client_id: 1,
            tx: 1,
            amount: Some(100.0),
            disputed: false,
        };
        let mut account: Account = Account {
            available: 0.0,
            held: 0.0,
            locked: false,
            transactions: vec![],
        };

        transaction_engine(&mut transaction, &mut account);

        assert_eq!(account.available, 100.0);
        assert_eq!(account.held, 0.0);
        assert_eq!(account.locked, false);
    }

    #[test]
    fn test_transaction_engine_withdrawal() {
        let mut transaction: Transaction = Transaction {
            tx_type: "withdrawal".to_string(),
            client_id: 1,
            tx: 1,
            amount: Some(50.0),
            disputed: false,
        };
        let mut account: Account = Account {
            available: 100.0,
            held: 0.0,
            locked: false,
            transactions: vec![],
        };

        transaction_engine(&mut transaction, &mut account);

        assert_eq!(account.available, 50.0);
        assert_eq!(account.held, 0.0);
        assert_eq!(account.locked, false);
    }

    #[test]
    fn test_transaction_engine_dispute() {
        let mut transaction: Transaction = Transaction {
            tx_type: "dispute".to_string(),
            client_id: 1,
            tx: 2,
            amount: None,
            disputed: false,
        };
        let mut account: Account = Account {
            available: 150.0,
            held: 0.0,
            locked: false,
            transactions: vec![
                Transaction {
                    tx_type: "deposit".to_string(),
                    client_id: 1,
                    tx: 1,
                    amount: Some(100.0),
                    disputed: false,
                },
                Transaction {
                    tx_type: "deposit".to_string(),
                    client_id: 1,
                    tx: 2,
                    amount: Some(50.0),
                    disputed: false,
                }
            ],
        };

        transaction_engine(&mut transaction, &mut account);

        assert_eq!(account.available, 100.0);
        assert_eq!(account.held, 50.0);
        assert_eq!(account.locked, false);
        assert_eq!(account.transactions[1].disputed, true);
    }

    #[test]
    fn test_transaction_engine_resolve() {
        let mut transaction: Transaction = Transaction {
            tx_type: "resolve".to_string(),
            client_id: 1,
            tx: 2,
            amount: None,
            disputed: false,
        };
        let mut account: Account = Account {
            available: 100.0,
            held: 50.0,
            locked: false,
            transactions: vec![
                Transaction {
                    tx_type: "deposit".to_string(),
                    client_id: 1,
                    tx: 1,
                    amount: Some(100.0),
                    disputed: false,
                },
                Transaction {
                    tx_type: "deposit".to_string(),
                    client_id: 1,
                    tx: 2,
                    amount: Some(50.0),
                    disputed: true,
                },
                Transaction {
                    tx_type: "dispute".to_string(),
                    client_id: 1,
                    tx: 2,
                    amount: None,
                    disputed: false,
                }
            ],
        };

        transaction_engine(&mut transaction, &mut account);

        assert_eq!(account.available, 150.0);
        assert_eq!(account.held, 0.0);
        assert_eq!(account.locked, false);
        assert_eq!(transaction.disputed, false);
    }

    #[test]
    fn test_transaction_engine_chargeback() {
        let mut transaction: Transaction = Transaction {
            tx_type: "chargeback".to_string(),
            client_id: 1,
            tx: 2,
            amount: None,
            disputed: false,
        };
        let mut account: Account = Account {
            available: 100.0,
            held: 50.0,
            locked: false,
            transactions: vec![
                Transaction {
                    tx_type: "deposit".to_string(),
                    client_id: 1,
                    tx: 1,
                    amount: Some(100.0),
                    disputed: false,
                },
                Transaction {
                    tx_type: "deposit".to_string(),
                    client_id: 1,
                    tx: 2,
                    amount: Some(50.0),
                    disputed: true,
                },
                Transaction {
                    tx_type: "dispute".to_string(),
                    client_id: 1,
                    tx: 2,
                    amount: None,
                    disputed: false,
                }
            ],
        };

        transaction_engine(&mut transaction, &mut account);

        assert_eq!(account.available, 100.0);
        assert_eq!(account.held, 0.0);
        assert_eq!(account.locked, true);
    }

    #[test]
    fn test_search_matching_deposit_transaction_dispute_false() {
        let mut account_transactions: Vec<Transaction> = vec![
            Transaction {
                tx_type: "deposit".to_string(),
                client_id: 1,
                tx: 1,
                amount: Some(100.0),
                disputed: false,
            },
            Transaction {
                tx_type: "deposit".to_string(),
                client_id: 1,
                tx: 2,
                amount: Some(50.0),
                disputed: false,
            },
        ];
        let result: Option<&mut Transaction> = search_matching_deposit_transaction(2, &mut account_transactions, false);
        match result {
            Some(transaction) => {
                assert_eq!(transaction.tx_type, "deposit".to_string());
                assert_eq!(transaction.client_id, 1);
                assert_eq!(transaction.tx, 2);
                assert_eq!(transaction.amount, Some(50.0));
                assert_eq!(transaction.disputed, false);
            },
            None => panic!("Matching deposit transaction not found"),
        }
    }

    #[test]
    fn test_search_matching_deposit_transaction_dispute_true() {
        let mut account_transactions: Vec<Transaction> = vec![
            Transaction {
                tx_type: "deposit".to_string(),
                client_id: 1,
                tx: 1,
                amount: Some(100.0),
                disputed: false,
            },
            Transaction {
                tx_type: "deposit".to_string(),
                client_id: 1,
                tx: 2,
                amount: Some(50.0),
                disputed: true,
            },
        ];
        let result: Option<&mut Transaction> = search_matching_deposit_transaction(2, &mut account_transactions, true);
        match result {
            Some(transaction) => {
                assert_eq!(transaction.tx_type, "deposit".to_string());
                assert_eq!(transaction.client_id, 1);
                assert_eq!(transaction.tx, 2);
                assert_eq!(transaction.amount, Some(50.0));
                assert_eq!(transaction.disputed, true);
            },
            None => panic!("Matching deposit transaction not found"),
        }
    }

    #[test]
    fn test_search_matching_disputed_transaction() {
        let mut account_transactions: Vec<Transaction> = vec![
            Transaction {
                tx_type: "deposit".to_string(),
                client_id: 1,
                tx: 1,
                amount: Some(100.0),
                disputed: false,
            },
            Transaction {
                tx_type: "deposit".to_string(),
                client_id: 1,
                tx: 2,
                amount: Some(50.0),
                disputed: true,
            },
            Transaction {
                tx_type: "dispute".to_string(),
                client_id: 1,
                tx: 2,
                amount: None,
                disputed: false,
            }
        ];
        let result: Option<&mut Transaction> = search_matching_disputed_transaction(2, &mut account_transactions);
        match result {
            Some(transaction) => {
                assert_eq!(transaction.tx_type, "deposit".to_string());
                assert_eq!(transaction.client_id, 1);
                assert_eq!(transaction.tx, 2);
                assert_eq!(transaction.amount, Some(50.0));
                assert_eq!(transaction.disputed, true);
            },
            None => panic!("Matching disputed transaction not found"),
        }
    }

    #[test]
    fn test_add_transaction_to_account() {
        let mut account: Account = Account {
            available: 100.0,
            held: 0.0,
            locked: false,
            transactions: vec![],
        };
        let transaction: Transaction = Transaction {
            tx_type: "deposit".to_string(),
            client_id: 1,
            tx: 1,
            amount: Some(100.0),
            disputed: false,
        };

        add_transaction_to_account(&mut account, &transaction);

        assert_eq!(account.transactions.len(), 1);
        let added_transaction: &Transaction = &account.transactions[0];
        assert_eq!(added_transaction.tx_type, "deposit".to_string());
        assert_eq!(added_transaction.client_id, 1);
        assert_eq!(added_transaction.tx, 1);
        assert_eq!(added_transaction.amount, Some(100.0));
        assert_eq!(added_transaction.disputed, false);
    }
}
