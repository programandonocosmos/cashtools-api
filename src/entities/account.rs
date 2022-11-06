use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Copy, Clone)]
pub enum EarningIndex {
    CDI,
    FIXED,
    IPCA,
}

#[derive(Copy, Clone)]
pub struct PreAllocation {
    pub amount: f64,
    pub accumulative: bool,
}

#[derive(Copy, Clone)]
pub struct Earning {
    pub rate: f64,
    pub index: EarningIndex,
}

pub struct Account {
    pub id: Uuid,
    pub time: NaiveDateTime,
    pub name: String,
    pub description: Option<String>,
    pub balance: f64,
    pub pre_allocation: Option<PreAllocation>,
    pub earning: Option<Earning>,
    pub is_available: bool,
    pub in_trash: bool,
}

pub struct NewAccount {
    pub time: NaiveDateTime,
    pub initial_balance: f64,
    pub name: String,
    pub description: Option<String>,
    pub pre_allocation: Option<PreAllocation>,
    pub earning: Option<Earning>,
    pub is_available: bool,
}
