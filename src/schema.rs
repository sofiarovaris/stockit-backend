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
    client_user (id) {
        id -> Integer,
        name -> Nullable<Text>,
        email -> Nullable<Text>,
        phone -> Nullable<Text>,
        state_registration -> Nullable<Text>,
        cpf -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        client_type -> Nullable<Text>,
        company_name -> Nullable<Text>,
        rg -> Nullable<Text>,
        address_id -> Nullable<Integer>,
        number -> Nullable<Integer>,
        complement -> Nullable<Text>,
    }
}

diesel::table! {
    employee (id) {
        id -> Integer,
        admission_date -> Nullable<Date>,
        payment_date -> Nullable<Date>,
        salary -> Nullable<Double>,
        comission -> Nullable<Double>,
        user_id -> Integer,
    }
}

diesel::table! {
    producer (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    product (id) {
        id -> Integer,
        name -> Nullable<Text>,
        code -> Nullable<Text>,
        observations -> Nullable<Text>,
        gross_weight -> Nullable<Double>,
        net_weight -> Nullable<Double>,
        batch_number -> Nullable<Text>,
        current_quantity -> Nullable<Integer>,
        provider_id -> Integer,
        producer_id -> Integer,
        unit_id -> Integer,
    }
}

diesel::table! {
    product_nfe (id) {
        id -> Integer,
        calculation_basis -> Nullable<Double>,
        icms_intern -> Nullable<Double>,
        ipi -> Nullable<Double>,
        origin -> Nullable<Text>,
        cest -> Nullable<Double>,
        ncm -> Nullable<Double>,
        csosn -> Nullable<Double>,
        product_id -> Integer,
    }
}

diesel::table! {
    product_price (id) {
        id -> Integer,
        price_cost -> Nullable<Double>,
        sale_price -> Nullable<Double>,
        profit_margin -> Nullable<Double>,
        product_id -> Integer,
    }
}

diesel::table! {
    provider (id) {
        id -> Integer,
        name -> Nullable<Text>,
        company_name -> Nullable<Text>,
        state_registration -> Nullable<Text>,
        email -> Nullable<Text>,
        phone -> Nullable<Text>,
        cnpj -> Nullable<Text>,
        bank_reference -> Nullable<Text>,
        address_id -> Nullable<Integer>,
        number -> Nullable<Integer>,
        complement -> Nullable<Text>,
    }
}

diesel::table! {
    role (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    unit (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    user (id) {
        id -> Integer,
        name -> Nullable<Text>,
        username -> Nullable<Text>,
        password -> Text,
        email -> Text,
        rg -> Nullable<Text>,
        cpf -> Nullable<Text>,
        phone -> Nullable<Text>,
        number -> Nullable<Integer>,
        complement -> Nullable<Text>,
        role_id -> Nullable<Integer>,
        address_id -> Nullable<Integer>,
    }
}

diesel::joinable!(client_user -> address (address_id));
diesel::joinable!(employee -> user (user_id));
diesel::joinable!(product -> producer (producer_id));
diesel::joinable!(product -> provider (provider_id));
diesel::joinable!(product -> unit (unit_id));
diesel::joinable!(product_nfe -> product (product_id));
diesel::joinable!(product_price -> product (product_id));
diesel::joinable!(provider -> address (address_id));
diesel::joinable!(user -> address (address_id));
diesel::joinable!(user -> role (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    address,
    client_user,
    employee,
    producer,
    product,
    product_nfe,
    product_price,
    provider,
    role,
    unit,
    user,
);
