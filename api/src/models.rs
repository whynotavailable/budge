use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

// This file is for database models.

#[derive(sqlx::Type, Serialize, Deserialize, Debug)]
#[sqlx(type_name = "TEXT")]
pub enum AccountType {
    Asset,
    Liability,
    Revenue,
    Expense,
    Capital,
}

impl AccountType {
    // This is nearly always going to be against money so i64
    fn direction(self) -> i64 {
        match self {
            AccountType::Asset => 1,
            AccountType::Liability => -1,
            AccountType::Capital => -1,
            AccountType::Revenue => -1,
            AccountType::Expense => 1,
        }
    }
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Account {
    pub name: String, // key
    pub account_type: AccountType,
    pub starting_balance: i64, // this is cents.
    pub last_clear: i64,
}

pub struct ledger {}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub id: Uuid, // will be a UUID
    pub date: i32,
    pub ordinal: i32,
}

pub struct TransactionEntry {}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct TransactionCategory {
    pub path: String,
    pub name: String,
    pub parent: String, // fk
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the parent module

    #[test]
    fn test_add() {
        // let at = AccountType::Asset;
        // let atd: AccountTypeDirection = at.into();
        // assert!(matches!(atd, AccountTypeDirection::Normal));
    }
}
