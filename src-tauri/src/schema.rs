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

diesel::table! {
    activity_times (id) {
        id -> Integer,
        start_time -> BigInt,
        end_time -> Nullable<BigInt>,
        activity_id -> Integer,
    }
}

diesel::joinable!(activity_times -> activities (activity_id));

diesel::allow_tables_to_appear_in_same_query!(
    activities,
    activity_times,
);
