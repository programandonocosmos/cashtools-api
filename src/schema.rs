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
