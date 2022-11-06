// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "earning_index_enum"))]
    pub struct EarningIndexEnum;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::EarningIndexEnum;

    accounts (id) {
        id -> Uuid,
        time -> Timestamp,
        name -> Text,
        description -> Nullable<Text>,
        last_calculated_balance -> Float8,
        is_pre_allocation -> Bool,
        pre_allocation_amount -> Nullable<Float8>,
        pre_allocation_accumulative -> Nullable<Bool>,
        is_earning -> Bool,
        earning_rate -> Nullable<Float8>,
        earning_index -> Nullable<EarningIndexEnum>,
        is_available -> Bool,
        in_trash -> Bool,
    }
}

diesel::table! {
    transactions (id) {
        id -> Uuid,
        related_user -> Uuid,
        entry_date -> Date,
        entry_account_code -> Nullable<Text>,
        exit_account_code -> Nullable<Text>,
        amount -> Float8,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    user_integrations (id) {
        id -> Uuid,
        related_user -> Uuid,
        name -> Text,
        time -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        register_date -> Nullable<Timestamp>,
        email -> Text,
        last_code_gen_request -> Nullable<Timestamp>,
        login_code -> Nullable<Int4>,
        is_registered -> Bool,
        name -> Text,
        payday -> Nullable<Int4>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    transactions,
    user_integrations,
    users,
);
