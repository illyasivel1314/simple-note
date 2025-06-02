// @generated automatically by Diesel CLI.

diesel::table! {
    execute_table (key) {
        key -> Char,
        timestamp -> BigInt,
    }
}

diesel::table! {
    calendar_table (date) {
        date -> Char,
        year -> Char,
        month -> Char,
        day -> Char,
        weekday -> Integer,
        date_type -> Integer,
        solar_term -> Char,
        lunar -> Char,
    }
}

diesel::table! {
    note_table (key) {
        key -> Char,
        content -> VarChar,
        create_time -> BigInt,
        tag_type -> Integer,
        start_time -> BigInt,
        end_time -> BigInt,
        finished_valid -> Integer,
        reminder_valid -> Integer,
        reminder_time -> Nullable<BigInt>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(execute_table, calendar_table, note_table,);
