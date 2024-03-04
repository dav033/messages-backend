diesel::table! {
    messages (id) {
        id -> Integer,
        body -> Text,
        #[max_length = 50]
        typeM -> Varchar,
        datetime -> Datetime,
        #[max_length = 100]
        sender -> Varchar,
        #[max_length = 100]
        receiver -> Varchar,
        context -> Nullable<Text>,
    }
}
