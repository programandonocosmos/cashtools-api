// @generated automatically by Diesel CLI.

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
    transactions,
    user_integrations,
    users,
);
