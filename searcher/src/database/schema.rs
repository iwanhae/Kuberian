// @generated automatically by Diesel CLI.

diesel::table! {
    function_analyses (id) {
        id -> Integer,
        function_id -> Integer,
        summary -> Text,
        background -> Nullable<Text>,
        analysis -> Nullable<Text>,
        purpose -> Nullable<Text>,
        comment -> Nullable<Text>,
        tldr -> Nullable<Text>,
    }
}

diesel::table! {
    functions (id) {
        id -> Integer,
        name -> Text,
        signature -> Text,
        file -> Text,
        code -> Text,
        line_start -> Integer,
        line_end -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(function_analyses, functions,);
