use std::{collections::HashMap, env};

mod utils;
mod ledger;
mod tests;

use crate::utils::{
    get_transactions,
    output_accounts,
    Transaction,
    Account,
};

use crate::ledger::process_transactions;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    let transactions: Vec<Transaction> = get_transactions(file_path);
    let accounts: HashMap<u16, Account> = process_transactions(transactions);
    output_accounts(accounts);
}
