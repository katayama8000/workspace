// @generated automatically by Diesel CLI.

diesel::table! {
    bird (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        scientific_name -> Varchar,
        #[max_length = 255]
        commonwealth_status -> Varchar,
    }
}
