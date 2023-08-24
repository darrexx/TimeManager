// @generated automatically by Diesel CLI.

diesel::table! {
    activities (id) {
        id -> Integer,
        name -> Text,
        duration -> Nullable<BigInt>,
        created_at -> BigInt,
        last_modified -> BigInt,
        workitem_id -> Nullable<BigInt>,
    }
}
