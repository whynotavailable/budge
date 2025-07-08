use uuid::Uuid;

pub struct Tenant {
    pub id: Uuid,
}

pub struct Account {
    pub tenant: Uuid,
    pub id: Option<Uuid>, // Should only be missing on creates.
    pub name: String,
    pub starting_balance: i64, // this is cents.
}

pub struct Transaction {
    pub tenant: Uuid,
    pub account_id: Uuid,
    pub id: Option<Uuid>, // will be a UUID
    pub date: u64,
    pub ord: i32, // pg has no u32
    pub category_path: String,
    pub amount: i64, // this is cents.
}

pub struct TransactionCategory {
    pub tenant: Uuid,
    pub path: String,
    pub name: String,
    pub parent: String, // fk
}
