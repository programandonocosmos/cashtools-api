use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_derive_enum;
use log;
use uuid::Uuid;

use crate::{database, entities::account, schema::accounts as account_schema};

#[derive(diesel_derive_enum::DbEnum, Debug, Clone, Copy)]
#[DieselTypePath = "crate::schema::sql_types::EarningIndexEnum"]
enum EarningIndexEnum {
    CDI,
    FIXED,
    IPCA,
}

#[derive(Queryable, Clone)]
#[diesel(table_name = account_schema)]
struct Account {
    id: Uuid,
    related_user: Uuid,
    time: NaiveDateTime,
    name: String,
    description: Option<String>,
    last_calculated_balance: f64,
    is_pre_allocation: bool,
    pre_allocation_amount: Option<f64>,
    pre_allocation_accumulative: Option<bool>,
    is_earning: bool,
    earning_rate: Option<f64>,
    earning_index: Option<EarningIndexEnum>,
    is_available: bool,
    in_trash: bool,
}

#[derive(Insertable, Clone, Debug)]
#[diesel(table_name = account_schema)]
struct NewAccount {
    related_user: Uuid,
    time: NaiveDateTime,
    name: String,
    description: Option<String>,
    last_calculated_balance: f64,
    is_pre_allocation: bool,
    pre_allocation_amount: Option<f64>,
    pre_allocation_accumulative: Option<bool>,
    is_earning: bool,
    earning_rate: Option<f64>,
    earning_index: Option<EarningIndexEnum>,
    is_available: bool,
    in_trash: bool,
}

#[derive(AsChangeset)]
#[diesel(table_name = account_schema)]
struct UpdatedAccount {
    name: Option<String>,
    description: Option<String>,
    is_pre_allocation: Option<bool>,
    pre_allocation_amount: Option<f64>,
    pre_allocation_accumulative: Option<bool>,
    is_earning: Option<bool>,
    earning_rate: Option<f64>,
    earning_index: Option<EarningIndexEnum>,
    is_available: Option<bool>,
    in_trash: Option<bool>,
}

impl account::EarningIndex {
    fn to_model(&self) -> EarningIndexEnum {
        match self {
            account::EarningIndex::CDI => EarningIndexEnum::CDI,
            account::EarningIndex::FIXED => EarningIndexEnum::FIXED,
            account::EarningIndex::IPCA => EarningIndexEnum::IPCA,
        }
    }
}

impl EarningIndexEnum {
    fn to_entity(&self) -> account::EarningIndex {
        match self {
            EarningIndexEnum::CDI => account::EarningIndex::CDI,
            EarningIndexEnum::FIXED => account::EarningIndex::FIXED,
            EarningIndexEnum::IPCA => account::EarningIndex::IPCA,
        }
    }
}

impl account::NewAccount {
    fn to_model(&self, related_user: Uuid) -> NewAccount {
        NewAccount {
            related_user,
            time: self.time,
            name: self.name.clone(),
            description: self.description.clone(),
            last_calculated_balance: 0.0,
            is_pre_allocation: match self.pre_allocation {
                Some(_) => true,
                None => false,
            },
            pre_allocation_amount: self.pre_allocation.map(|x| x.amount),
            pre_allocation_accumulative: self.pre_allocation.map(|x| x.accumulative),
            is_earning: match self.earning {
                Some(_) => true,
                None => false,
            },
            earning_rate: self.earning.map(|x| x.rate),
            earning_index: self.earning.map(|x| x.index.to_model()),
            is_available: self.is_available,
            in_trash: false,
        }
    }
}

fn pre_allocation_from_table_fields(
    is_pre_allocation: bool,
    amount: Option<f64>,
    accumulative: Option<bool>,
) -> Option<account::PreAllocation> {
    match (is_pre_allocation, amount, accumulative) {
        (true, Some(amount), Some(accumulative)) => Some(account::PreAllocation {
            amount,
            accumulative,
        }),
        _ => None,
    }
}

fn earning_from_table_fields(
    is_earning: bool,
    rate: Option<f64>,
    index: Option<account::EarningIndex>,
) -> Option<account::Earning> {
    match (is_earning, rate, index) {
        (true, Some(rate), Some(index)) => Some(account::Earning { rate, index }),
        _ => None,
    }
}

impl Account {
    fn to_entity(&self) -> account::Account {
        let pre_allocation = pre_allocation_from_table_fields(
            self.is_pre_allocation,
            self.pre_allocation_amount,
            self.pre_allocation_accumulative,
        );

        let earning = earning_from_table_fields(
            self.is_earning,
            self.earning_rate,
            self.earning_index.map(|x| x.to_entity()),
        );

        account::Account {
            id: self.id,
            time: self.time,
            name: self.name.clone(),
            description: self.description.clone(),
            balance: self.last_calculated_balance,
            pre_allocation,
            earning,
            is_available: self.is_available,
            in_trash: self.in_trash,
        }
    }
}

impl account::UpdatedAccount {
    fn to_model(&self) -> UpdatedAccount {
        UpdatedAccount {
            name: self.name.clone(),
            description: self.description.clone(),
            is_pre_allocation: match self.pre_allocation {
                Some(_) => Some(true),
                None => None,
            },
            pre_allocation_amount: self.pre_allocation.map(|x| x.amount),
            pre_allocation_accumulative: self.pre_allocation.map(|x| x.accumulative),
            is_earning: match self.earning {
                Some(_) => Some(true),
                None => None,
            },
            earning_rate: self.earning.map(|x| x.rate),
            earning_index: self.earning.map(|x| x.index.to_model()),
            is_available: self.is_available,
            in_trash: self.in_trash,
        }
    }
}

impl account::AccountModel for database::DbPool {
    fn create_account(
        &self,
        user_id: Uuid,
        new_account: account::NewAccount,
    ) -> account::Result<account::Account> {
        let parsed_account = new_account.to_model(user_id);
        log::debug!("Parsed account: {:?}", parsed_account);
        diesel::insert_into(account_schema::table)
            .values(&parsed_account)
            .get_result::<Account>(&mut self.get()?)
            .map(|t| t.to_entity())
            .map_err(account::AccountModelError::FailedToCreateAccount)
    }
    fn get_account(&self, id: &Uuid, user_id: &Uuid) -> account::Result<account::Account> {
        let accounts = account_schema::table
            .filter(account_schema::related_user.eq(user_id))
            .filter(account_schema::id.eq(id))
            .load::<Account>(&mut self.get()?)
            .map_err(account::AccountModelError::FailedToGetAccount)?;

        match accounts.as_slice() {
            [acc] => Ok(acc.to_entity()),
            [] => Err(account::AccountModelError::AccountNotFound),
            _ => Err(account::AccountModelError::MultipleAccountWithSameId),
        }
    }
    fn get_accounts(&self, user_id: &Uuid) -> account::Result<Vec<account::Account>> {
        let accounts = account_schema::table
            .filter(account_schema::related_user.eq(user_id))
            .load::<Account>(&mut self.get()?)
            .map_err(account::AccountModelError::FailedToGetAccount)?;

        Ok(accounts.iter().map(|t| t.to_entity()).collect())
    }

    fn delete_account(&self, id: &Uuid, user_id: &Uuid) -> account::Result<()> {
        match self.edit_account(
            id,
            user_id,
            account::UpdatedAccount {
                name: None,
                description: None,
                pre_allocation: None,
                earning: None,
                is_available: None,
                in_trash: Some(true),
            },
        ) {
            Ok(_) => Ok(()),
            Err(err) => Err(account::AccountModelError::FailedToDeleteAccount(Box::new(
                err,
            ))),
        }
    }

    fn edit_account(
        &self,
        id: &Uuid,
        user_id: &Uuid,
        updated_account: account::UpdatedAccount,
    ) -> account::Result<account::Account> {
        let account = diesel::update(
            account_schema::table
                .filter(account_schema::id.eq(id))
                .filter(account_schema::related_user.eq(user_id)),
        )
        .set(updated_account.to_model())
        .get_result::<Account>(&mut self.get()?)
        .map_err(account::AccountModelError::FailedToUpdateAccount)?;
        Ok(account.to_entity())
    }
}
