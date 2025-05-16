// @generated automatically by Diesel CLI.

diesel::table! {
    execute_table (id) {
        id -> Integer,
        key -> Char,
        timestamp -> BigInt,
        finished -> Integer,
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
        #[sql_name = "type"]
        category -> Integer,
        start_time -> BigInt,
        end_time -> Nullable<BigInt>,
        cycle -> Nullable<Char>,
        version -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(execute_table, calendar_table, note_table,);
