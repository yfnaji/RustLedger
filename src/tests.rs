mod functional_tests {
    use crate::*;

    #[allow(dead_code)]
    fn check_account(account: &Account, available: f32, held: f32, locked: bool) {
        assert_eq!(account.available, available);
        assert_eq!(account.held, held);
        assert_eq!(account.locked, locked);
    }

    #[test]
    fn test_deposit_then_withdrawal() {
        let transactions: Vec<crate::utils::Transaction> = vec![
            crate::utils::Transaction {
                tx_type: "deposit".to_string(),
                client_id: 1,
                tx: 1,
                amount: Some(100.0),
                disputed: false,
            },
            crate::utils::Transaction {
                tx_type: "withdrawal".to_string(),
                client_id: 1,
                tx: 2,
                amount: Some(50.0),
                disputed: false,
            },
        ];
        let accounts: HashMap<u16, Account> = process_transactions(transactions);
        let account: &Account = accounts.get(&1).unwrap();

        check_account(account, 50.0, 0.0, false);
    }

    #[test]
    fn test_deposit_with_larger_withdrawal() {
        let transactions: Vec<Transaction> = vec![
            Transaction {
                tx_type: "deposit".to_string(),
                client_id: 1,
                tx: 1,
                amount: Some(100.0),
                disputed: false,
            },
            Transaction {
                tx_type: "withdrawal".to_string(),
                client_id: 1,
                tx: 2,
                amount: Some(200.0),
                disputed: false,
            },
        ];
        let accounts: HashMap<u16, Account> = process_transactions(transactions);
        let account: &Account = accounts.get(&1).unwrap();

        check_account(account, 100.0, 0.0, false);
    }

    #[test]
    fn test_dispute_on_withdrawal() {
        let transactions: Vec<Transaction> = vec![
            Transaction {
                tx_type: "deposit".to_string(),
                client_id: 1,
                tx: 1,
                amount: Some(100.0),
                disputed: false,
            },
            Transaction {
                tx_type: "withdrawal".to_string(),
                client_id: 1,
                tx: 2,
                amount: Some(50.0),
                disputed: false,
            },
            Transaction {
                tx_type: "dispute".to_string(),
                client_id: 1,
                tx: 2,
                amount: None,
                disputed: false,
            },
        ];
        let accounts: HashMap<u16, Account> = process_transactions(transactions);
        let account: &Account = accounts.get(&1).unwrap();

        check_account(account, 50.0, 0.0, false);
    }

    #[test]
    fn test_dispute_on_already_disputed_deposit() {
        let transactions: Vec<Transaction> = vec![
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
                tx_type: "dispute".to_string(),
                client_id: 1,
                tx: 1,
                amount: None,
                disputed: false,
            },
        ];
        let accounts: HashMap<u16, Account> = process_transactions(transactions);
        let account: &Account = accounts.get(&1).unwrap();

        check_account(account, 0.0, 100.0, false);
    }

    #[test]
    fn test_dispute_on_nonexistent_transaction() {
        let transactions: Vec<Transaction> = vec![
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
                tx: 2,
                amount: None,
                disputed: false,
            },
        ];
        let accounts: HashMap<u16, Account> = process_transactions(transactions);
        let account: &Account = accounts.get(&1).unwrap();

        check_account(account, 100.0, 0.0, false);
    }

    #[test]
    fn test_resolve_on_non_disputed_transaction() {
        let transactions: Vec<Transaction> = vec![
            Transaction {
                tx_type: "deposit".to_string(),
                client_id: 1,
                tx: 1,
                amount: Some(100.0),
                disputed: false,
            },
            Transaction {
                tx_type: "resolve".to_string(),
                client_id: 1,
                tx: 1,
                amount: None,
                disputed: false,
            },
        ];
        let accounts: HashMap<u16, Account> = process_transactions(transactions);
        let account: &Account = accounts.get(&1).unwrap();

        check_account(account, 100.0, 0.0, false);
    }

    #[test]
    fn test_resolve_on_non_existent_transaction() {
        let transactions: Vec<Transaction> = vec![
            Transaction {
                tx_type: "deposit".to_string(),
                client_id: 1,
                tx: 1,
                amount: Some(100.0),
                disputed: false,
            },
            Transaction {
                tx_type: "resolve".to_string(),
                client_id: 1,
                tx: 2,
                amount: None,
                disputed: false,
            },
        ];
        let accounts: HashMap<u16, Account> = process_transactions(transactions);
        let account: &Account = accounts.get(&1).unwrap();

        check_account(account, 100.0, 0.0, false);
    }

    #[test]
    fn test_on_non_disputed_transaction() {
        let transactions: Vec<Transaction> = vec![
            Transaction {
                tx_type: "deposit".to_string(),
                client_id: 1,
                tx: 1,
                amount: Some(100.0),
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
        let accounts: HashMap<u16, Account> = process_transactions(transactions);
        let account: &Account = accounts.get(&1).unwrap();

        check_account(account, 100.0, 0.0, false);
    }

    #[test]
    fn test_chargeback_on_non_existent_transaction() {
        let transactions: Vec<Transaction> = vec![
            Transaction {
                tx_type: "deposit".to_string(),
                client_id: 1,
                tx: 1,
                amount: Some(100.0),
                disputed: false,
            },
            Transaction {
                tx_type: "chargeback".to_string(),
                client_id: 1,
                tx: 2,
                amount: None,
                disputed: false,
            },
        ];
        let accounts: HashMap<u16, Account> = process_transactions(transactions);
        let account: &Account = accounts.get(&1).unwrap();

        check_account(account, 100.0, 0.0, false);
    }

    #[test]
    fn test_dispute_with_insufficient_funds() {
        let transactions: Vec<Transaction> = vec![
            Transaction {
                tx_type: "deposit".to_string(),
                client_id: 1,
                tx: 1,
                amount: Some(100.0),
                disputed: false,
            },
            Transaction {
                tx_type: "withdrawal".to_string(),
                client_id: 1,
                tx: 2,
                amount: Some(50.0),
                disputed: false,
            },
            Transaction {
                tx_type: "dispute".to_string(),
                client_id: 1,
                tx: 1,
                amount: None,
                disputed: false,
            },
        ];
        let accounts: HashMap<u16, Account> = process_transactions(transactions);
        let account: &Account = accounts.get(&1).unwrap();

        check_account(account, 50.0, 0.0, false);
    }

    #[test]
    fn test_two_deposits_both_disputed() {
        let transactions: Vec<Transaction> = vec![
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
            Transaction {
                tx_type: "dispute".to_string(),
                client_id: 1,
                tx: 1,
                amount: None,
                disputed: false,
            },
            Transaction {
                tx_type: "dispute".to_string(),
                client_id: 1,
                tx: 2,
                amount: None,
                disputed: false,
            },
        ];
        let accounts: HashMap<u16, Account> = process_transactions(transactions);
        let account: &Account = accounts.get(&1).unwrap();

        check_account(account, 0.0, 150.0, false);
    }

    #[test]
    fn test_two_deposits_both_resolved() {
        let transactions: Vec<Transaction> = vec![
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
            Transaction {
                tx_type: "dispute".to_string(),
                client_id: 1,
                tx: 1,
                amount: None,
                disputed: false,
            },
            Transaction {
                tx_type: "dispute".to_string(),
                client_id: 1,
                tx: 2,
                amount: None,
                disputed: false,
            },
            Transaction {
                tx_type: "resolve".to_string(),
                client_id: 1,
                tx: 1,
                amount: None,
                disputed: false,
            },
            Transaction {
                tx_type: "resolve".to_string(),
                client_id: 1,
                tx: 2,
                amount: None,
                disputed: false,
            },
        ];
        let accounts: HashMap<u16, Account> = process_transactions(transactions);
        let account: &Account = accounts.get(&1).unwrap();

        check_account(account, 150.0, 0.0, false);
    }

    #[test]
    fn test_two_disputes_one_resolved_one_chargeback() {
        let transactions: Vec<Transaction> = vec![
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
            Transaction {
                tx_type: "dispute".to_string(),
                client_id: 1,
                tx: 1,
                amount: None,
                disputed: false,
            },
            Transaction {
                tx_type: "dispute".to_string(),
                client_id: 1,
                tx: 2,
                amount: None,
                disputed: false,
            },
            Transaction {
                tx_type: "resolve".to_string(),
                client_id: 1,
                tx: 1,
                amount: None,
                disputed: false,
            },
            Transaction {
                tx_type: "chargeback".to_string(),
                client_id: 1,
                tx: 2,
                amount: None,
                disputed: false,
            },
        ];
        let accounts: HashMap<u16, Account> = process_transactions(transactions);
        let account: &Account = accounts.get(&1).unwrap();

        check_account(account, 100.0, 0.0, true);
    }

    #[test]
    fn test_no_recorded_transactions_after_chargeback() {
        let transactions: Vec<Transaction> = vec![
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
            Transaction {
                tx_type: "deposit".to_string(),
                client_id: 1,
                tx: 1,
                amount: Some(1000.0),
                disputed: false,
            },

        ];
        let accounts: HashMap<u16, Account> = process_transactions(transactions);
        let account: &Account = accounts.get(&1).unwrap();

        check_account(account, 0.0, 0.0, true);
        assert_eq!(account.transactions.len(), 3);
    }

    #[test]
    fn test_multi_client_transactions() {
        let transactions: Vec<Transaction> = vec![
            Transaction {
                tx_type: "deposit".to_string(),
                client_id: 1,
                tx: 1,
                amount: Some(100.0),
                disputed: false,
            },
            Transaction {
                tx_type: "deposit".to_string(),
                client_id: 2,
                tx: 2,
                amount: Some(200.0),
                disputed: false,
            },
            Transaction {
                tx_type: "withdrawal".to_string(),
                client_id: 1,
                tx: 3,
                amount: Some(50.0),
                disputed: false,
            },
            Transaction {
                tx_type: "withdrawal".to_string(),
                client_id: 2,
                tx: 4,
                amount: Some(100.0),
                disputed: false,
            },
            Transaction {
                tx_type: "deposit".to_string(),
                client_id: 1,
                tx: 5,
                amount: Some(25.0),
                disputed: false,
            },
            Transaction {
                tx_type: "deposit".to_string(),
                client_id: 2,
                tx: 6,
                amount: Some(50.0),
                disputed: false,
            },
            Transaction {
                tx_type: "dispute".to_string(),
                client_id: 1,
                tx: 5,
                amount: None,
                disputed: false,
            },
            Transaction {
                tx_type: "dispute".to_string(),
                client_id: 2,
                tx: 6,
                amount: None,
                disputed: false,
            },
            Transaction {
                tx_type: "resolve".to_string(),
                client_id: 1,
                tx: 5,
                amount: None,
                disputed: false,
            },
            Transaction {
                tx_type: "chargeback".to_string(),
                client_id: 2,
                tx: 6,
                amount: None,
                disputed: false,
            },
        ];
        let accounts: HashMap<u16, Account> = process_transactions(transactions);
        let account1: &Account = accounts.get(&1).unwrap();
        let account2: &Account = accounts.get(&2).unwrap();

        check_account(account1, 75.0, 0.0, false);
        check_account(account2, 100.0, 0.0, true);
    }

    #[test]
    fn test_floating_point_precision() {
        let transactions: Vec<Transaction> = vec![
            Transaction {
                tx_type: "deposit".to_string(),
                client_id: 1,
                tx: 1,
                amount: Some(100.5555),
                disputed: false,
            },
            Transaction {
                tx_type: "deposit".to_string(),
                client_id: 1,
                tx: 2,
                amount: Some(20.2222),
                disputed: false,
            },
            Transaction {
                tx_type: "withdrawal".to_string(),
                client_id: 1,
                tx: 3,
                amount: Some(50.1111),
                disputed: false,
            },
        ];
        let accounts: HashMap<u16, Account> = process_transactions(transactions);
        let account: &Account = accounts.get(&1).unwrap();

        assert_eq!((account.available * 10000.0).round() / 10000.0, 70.6666);
        assert_eq!(account.held, 0.0);
        assert_eq!(account.locked, false);
    }

    #[test]
    fn test_empty_deposit_amount() {
        let transactions: Vec<Transaction> = vec![
            Transaction {
                tx_type: "deposit".to_string(),
                client_id: 1,
                tx: 1,
                amount: None,
                disputed: false,
            },
        ];
        let accounts: HashMap<u16, Account> = process_transactions(transactions);
        let account: &Account = accounts.get(&1).unwrap();

        check_account(account, 0.0, 0.0, false);
    }

    #[test]
    fn fn_test_empty_withdrawal_amount() {
        let transactions: Vec<Transaction> = vec![
            Transaction {
                tx_type: "deposit".to_string(),
                client_id: 1,
                tx: 1,
                amount: Some(100.0),
                disputed: false,
            },
            Transaction {
                tx_type: "withdrawal".to_string(),
                client_id: 1,
                tx: 2,
                amount: None,
                disputed: false,
            },
        ];
        let accounts: HashMap<u16, Account> = process_transactions(transactions);
        let account: &Account = accounts.get(&1).unwrap();

        check_account(account, 100.0, 0.0, false);
    }
}
