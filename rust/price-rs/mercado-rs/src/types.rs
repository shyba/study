use serde::{Deserialize, Serialize};

// todo: find a better name.. this will probably be something else later
pub struct RequestBuilder {
    pub base_url: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Paging {
    total: u64,
    primary_results: u32,
    offset: u32,
    limit: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Seller {
    id: u64,
    permalink: String,
    registration_date: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Installments {
    quantity: u8,
    amount: f32,
    rate: f32,
    currency_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Address {
    state_id: String,
    state_name: String,
    city_id: String,
    city_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Shipping {
    free_shipping: bool,
    mode: String,
    tags: Vec<String>,
    logistic_type: String,
    store_pick_up: bool
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Attribute {
    id: String,
    attribute_group_id: String,
    name: String,
    value_name: Option<String>,
    attribute_group_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SearchResult {
    id: String,
    title: String,
    seller: Seller,
    price: f32,
    available_quantity: u32,
    sold_quantity: u32,
    condition: String, // should be enum
    permalink: String,
    thumbnail: String,
    accepts_mercadopago: bool,
    installments: Installments,
    address: Address,
    shipping: Shipping,
    category_id: String,
    tags: Vec<String>,
    domain_id: Option<String>,
    attributes: Vec<Attribute>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Sort {
    id: String,
    name: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SearchResults {
    query: String,
    paging: Paging,
    results: Vec<SearchResult>,
    sort: Sort,
    available_sorts: Vec<Sort>
}