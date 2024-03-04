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

diesel::table! {
    bird_sighting (id) {
        id -> Integer,
        bird_id -> Integer,
        sighting_date -> Nullable<Timestamp>,
        #[max_length = 255]
        sighting_location -> Nullable<Varchar>,
        additional_information -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    bird,
    bird_sighting,
);
