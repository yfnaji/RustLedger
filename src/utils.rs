use std::collections::HashMap;
use csv::{ReaderBuilder, StringRecord, Reader};

pub struct Transaction {
    pub tx_type: String,
    pub client_id: u16,
    pub tx: u32,
    pub amount: Option<f32>,
    pub disputed: bool,
}

impl Clone for Transaction {
    fn clone(&self) -> Self {
        Transaction {
            tx_type: self.tx_type.clone(),
            client_id: self.client_id,
            tx: self.tx,
            amount: self.amount,
            disputed: self.disputed,
        }
    }
}

pub struct Account {
    pub available: f32,
    pub held: f32,
    pub locked: bool,
    pub transactions: Vec<Transaction>,
}

pub fn get_transactions(file_path: &str) -> Vec<Transaction> {
    let mut rdr: Reader<std::fs::File> = ReaderBuilder::new().from_path(file_path).unwrap();
    let mut transactions: Vec<Transaction> = vec![];
    
    for result in rdr.records() {
        match result {
            Ok(rcd) => {
                    let (tx_type, client_id, tx, amount) = match process_row(rcd) {
                    Ok(data) => data,
                    Err(_) => continue,
                };
                transactions.push(
                    Transaction {
                        tx_type,
                        client_id,
                        tx,
                        amount,
                        disputed: false,
                    }
                );
            }
            Err(_) => continue,
        }
    }

    transactions
}

fn process_row(result: StringRecord) -> Result<(String, u16, u32, Option<f32>), ()> {
    let record: StringRecord = result;
    let tx_type: String = get_tx_type(&record)?;
    let client_id: u16 = get_client_id(&record)?;
    let tx: u32 = get_tx(&record)?;
    let amount: Option<f32>= get_amount(&record)?;
    
    Ok((tx_type, client_id, tx, amount))
}

fn get_tx_type(record: &StringRecord) -> Result<String, ()> {
    match record.get(0) {
        Some(tx_type) => Ok(tx_type.trim().to_string()),
        None => Err(()),
    }
}

fn get_client_id(record: &StringRecord) -> Result<u16, ()> {
    match record.get(1) {
        Some(id) => {
            match id.trim().parse() {
                Ok(numeric_id) => Ok(numeric_id),
                Err(_) => Err(()),

            }
        },
        None => Err(()),
    }
}

fn get_tx(record: &StringRecord) -> Result<u32, ()> {
    match record.get(2) {
        Some(id) => {
            match id.trim().parse() {
                Ok(numeric_id) => Ok(numeric_id),
                Err(_) => Err(()),

            }
        },
        None => Err(()),
    }
}

fn get_amount(record: &StringRecord) -> Result<Option<f32>, ()> {
    match record.get(3) {
        Some(amount_str) if !amount_str.is_empty() => {
            match amount_str.trim().parse() {
                Ok(amount) => {
                    if amount < 0.0 {
                        return Err(());
                    }
                    Ok(Some(amount))
                },
                Err(_) => Err(()),
            }
        },
        _ => Ok(None),
    }
}

pub fn output_accounts(accounts: HashMap<u16, Account>) {
    println!("client,available,held,total,locked");
    for (client_id, account) in accounts.iter() {
        let total: f32 = account.available + account.held;
        println!(
            "{},{:.4},{:.4},{:.4},{}", 
            client_id,
            account.available, 
            account.held, 
            total, 
            account.locked
        );
    }
}
