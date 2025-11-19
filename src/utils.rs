use std::collections::HashMap;
use csv::{StringRecord};

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

pub fn process_row(result: StringRecord) -> Result<(String, u16, u32, Option<f32>), ()> {
    let record: StringRecord = result;
    let tx_type: String = get_string(&record)?;
    let client_id: u16 = get_int_u16(&record, 1)?;
    let tx: u32 = get_int_u32(&record, 2)?;
    let amount: Option<f32>= get_float(&record)?;
    
    Ok((tx_type, client_id, tx, amount))
}

fn get_string(record: &StringRecord) -> Result<String, ()> {
    match record.get(0) {
        Some(tx_type) => Ok(tx_type.trim().to_string()),
        None => Err(()),
    }
}

fn get_int_u16(record: &StringRecord, index: usize) -> Result<u16, ()> {
    match record.get(index) {
        Some(id) => {
            match id.trim().parse() {
                Ok(numeric_id) => Ok(numeric_id),
                Err(_) => Err(()),

            }
        },
        None => Err(()),
    }
}

fn get_int_u32(record: &StringRecord, index: usize) -> Result<u32, ()> {
    match record.get(index) {
        Some(id) => {
            match id.trim().parse() {
                Ok(numeric_id) => Ok(numeric_id),
                Err(_) => Err(()),

            }
        },
        None => Err(()),
    }
}


fn get_float(record: &StringRecord) -> Result<Option<f32>, ()> {
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
