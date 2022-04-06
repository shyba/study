struct Paging {
    total: u64,
    primary_results: u32,
    offset: u32,
    limit: u32,
}

struct Seller {
    id: String,
    permalink: String,
    registration_date: String,
}

struct Installments {
    quantity: u8,
    amount: u32,
    rate: f32,
    currency_id: String,
}

struct Address {
    state_id: String,
    state_name: String,
    city_id: String,
    city_name: String,
}

struct Shipping {
    free_shipping: bool,
    mode: String,
    tags: Vec<String>,
    logistic_type: String,
    store_pick_up: bool
}

struct Attribute {
    id: String,
    attribute_group_id: String,
    name: String,
    value_name: String,
    attribute_group_name: String,
}

struct SearchResult {
    id: String,
    title: String,
    seller: Seller,
    price: u64,
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

struct Sort {
    id: String,
    name: String
}

struct SearchResults {
    query: String,
    paging: Paging,
    results: Vec<SearchResult>,
    sort: Sort,
    available_sorts: Vec<Sort>
}