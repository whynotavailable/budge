use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

// This file is for database models.

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
    pub starting_balance: i64, // this is cents.
    pub last_clear: i64,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Clears {
    pub account_id: Uuid,
    pub date: i32,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub account_id: Uuid,
    pub id: Uuid, // will be a UUID
    pub date: i32,
    pub ordinal: i32,
    pub category_path: String,
    pub amount: i64, // this is cents.
    pub cleard: bool,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct TransactionCategory {
    pub path: String,
    pub name: String,
    pub parent: String, // fk
}
