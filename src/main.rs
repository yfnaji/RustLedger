use std::{collections::HashMap, env};

mod utils;
mod ledger;
mod tests;

use crate::utils::{
    output_accounts,
    Account,
};
use crate::ledger::get_transactions;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    // let transactions: Vec<Transaction> = get_transactions(file_path);
    let accounts: HashMap<u16, Account> = get_transactions(file_path);
    output_accounts(accounts);
}
