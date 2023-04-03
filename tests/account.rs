use cashtools::entities::account::{AccountModel, Earning, EarningIndex, NewAccount};
mod common;
use env_logger;
use uuid::Uuid;

#[test]
fn create_and_delete_account() {
    let conn = common::make_conn();
    let user_id: Uuid = Uuid::from_u128(160141200314647599499076565412518613020);
    let new_account = NewAccount {
        time: common::now(),
        initial_balance: 15.0,
        name: format!("test account - {}", Uuid::new_v4()),
        description: Some("test account description".to_string()),
        pre_allocation: None,
        earning: None,
        is_available: false,
    };
    let account = conn.create_account(user_id, new_account).unwrap();
    conn.delete_account(&account.id, &user_id).unwrap();
}

#[test]
fn create_and_delete_account_with_earning() {
    env_logger::init();
    let conn = common::make_conn();
    let user_id: Uuid = Uuid::from_u128(160141200314647599499076565412518613020);
    let new_account = NewAccount {
        time: common::now(),
        initial_balance: 15.0,
        name: format!("test account - {}", Uuid::new_v4()),
        description: Some("test account description".to_string()),
        pre_allocation: None,
        earning: Some(Earning {
            rate: 15.0,
            index: EarningIndex::CDI,
        }),
        is_available: false,
    };
    let account = conn.create_account(user_id, new_account).unwrap();
    conn.delete_account(&account.id, &user_id).unwrap();
}
