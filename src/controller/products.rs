use dioxus::Result as DxResult;
use graphql_client::{GraphQLQuery, QueryBody, Response};
use reqwest::{get, Client};
use tracing::info;

// # Modules
use super::Controller;
use crate::{
    model::{
        pagination::PageSort,
        products::{
            products_query::{ProductsQueryProductsNodes, ResponseData, Variables},
            Product, Products, ProductsQuery,
        },
    },
    State,
};

pub(crate) async fn fetch_product(product_id: usize) -> DxResult<Product> {
    let url: String = format!("https://fakestoreapi.com/products/{product_id}");
    let request: Product = get(&url).await?.json().await?;

    Ok(request)
}

pub(crate) async fn _fetch_products(count: usize) -> DxResult<Vec<Product>> {
    let url: String = format!("https://fakestoreapi.com/products/?sort=ASC&limit={count}");

    let request: Vec<Product> = get(&url).await?.json().await?;

    Ok(request)
}

impl Controller for Products {
    type ReturnedEntity = Self;

    /// # Fetch a page of products
    ///
    /// Get a list of products from the WordPress GraphQL API.  
    ///
    /// **Arguments**  
    /// * `page_size` - The number of products to fetch.
    /// * `sort_direction` - The direction to sort the products.
    ///
    /// **Returns**  
    /// A list of products.
    async fn get_page(
        page_size: Option<usize>,
        _sort_direction: Option<PageSort>,
    ) -> DxResult<Self> {
        // Build the payload
        let first: i64 = page_size.unwrap_or(Self::PAGE_SIZE) as i64;
        let payload: Variables = Variables { first, after: None };
        let payload: QueryBody<Variables> = ProductsQuery::build_query(payload);

        // Build the endpoint
        let endpoint: String = format!(
            "{host}/{path}",
            host = State::get_backend_host(),
            path = State::get_backend_path()
        );

        info!("Fetching products from {endpoint}");

        // Make the request
        let request = Client::new().post(endpoint).json(&payload).send().await?;

        info!("Received response from {request:?}");

        // Parse the response
        let products: Vec<ProductsQueryProductsNodes> = request
            // Parse the response
            .json::<Response<ResponseData>>()
            // Get the products
            .await?
            .data
            .expect("No data received")
            .products
            .expect("No products received")
            .nodes;

        // Build and return the products
        Ok(products.into())
    }
}
