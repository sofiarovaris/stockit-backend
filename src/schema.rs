// @generated automatically by Diesel CLI.

diesel::table! {
    address (id) {
        id -> Integer,
        cep -> Text,
        street -> Text,
        city -> Text,
        state -> Text,
        neighborhood -> Text,
    }
}

diesel::table! {
    client (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        phone -> Text,
        cell -> Text,
        cpf -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Text,
        company_name -> Nullable<Text>,
        rg -> Nullable<Text>,
        address_id -> Integer,
    }
}

diesel::table! {
    comission_history (id) {
        id -> Integer,
        comission -> Double,
        date -> Date,
        employee_id -> Integer,
    }
}

diesel::table! {
    employee (id) {
        id -> Integer,
        admission_date -> Date,
        payment_date -> Date,
        user_id -> Integer,
    }
}

diesel::table! {
    payment (id) {
        id -> Integer,
        amount -> Double,
        month -> Double,
        year -> Integer,
        employee_id -> Integer,
    }
}

diesel::table! {
    provider (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        phone -> Text,
        cnpj -> Nullable<Text>,
        company_name -> Nullable<Text>,
        state_registration -> Nullable<Text>,
        address_id -> Integer,
        bank_reference -> Nullable<Text>,
    }
}

diesel::table! {
    salary_history (id) {
        id -> Integer,
        salary -> Double,
        date -> Date,
        employee_id -> Integer,
    }
}

diesel::table! {
    user (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        rg -> Text,
        cpf -> Text,
        phone -> Text,
        username -> Text,
        password -> Text,
        number -> Integer,
        complement -> Nullable<Text>,
        address_id -> Integer,
    }
}

diesel::joinable!(client -> address (address_id));
diesel::joinable!(comission_history -> employee (employee_id));
diesel::joinable!(employee -> user (user_id));
diesel::joinable!(payment -> employee (employee_id));
diesel::joinable!(provider -> address (address_id));
diesel::joinable!(salary_history -> employee (employee_id));
diesel::joinable!(user -> address (address_id));

diesel::allow_tables_to_appear_in_same_query!(
    address,
    client,
    comission_history,
    employee,
    payment,
    provider,
    salary_history,
    user,
);
