mod utils;
mod ledger;
mod tests;

use std::{collections::HashMap, env};
use crate::utils::{
    output_accounts,
    Account,
};
use crate::ledger::summarize_accounts;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    let accounts: HashMap<u16, Account> = summarize_accounts(file_path);
    output_accounts(accounts);
}
