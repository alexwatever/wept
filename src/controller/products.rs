use dioxus::Result as DxResult;
use graphql_client::Response;
use reqwest::get;
use tracing::info;

// # Modules
use super::{base::GraphQLEntity, Controller};
use crate::model::{
    pagination::PageSort,
    products::{
        products_query::{ProductsQueryProductsNodes, ResponseData, Variables},
        Product, Products, ProductsQuery,
    },
};

/// # Fetch a single product by ID
///
/// Get a product from the API by its ID.
///
/// **Arguments**
/// * `product_id` - The ID of the product to fetch.
///
/// **Returns**
/// A product.
pub(crate) async fn fetch_product(product_id: usize) -> DxResult<Product> {
    let url: String = format!("https://fakestoreapi.com/products/{product_id}");
    let request: Product = get(&url).await?.json().await?;

    Ok(request)
}

/// # Fetch a list of products
///
/// Get a list of products from the API.
///
/// **Arguments**
/// * `count` - The number of products to fetch.
///
/// **Returns**
/// A list of products.
#[allow(dead_code)]
pub(crate) async fn fetch_products(count: usize) -> DxResult<Vec<Product>> {
    let url: String = format!("https://fakestoreapi.com/products/?sort=ASC&limit={count}");
    let request: Vec<Product> = get(&url).await?.json().await?;
    Ok(request)
}

// Implement GraphQLEntity trait for Products
impl GraphQLEntity for Products {
    type Variables = Variables;
    type Query = ProductsQuery;
    type ResponseData = ResponseData;
    type Nodes = ProductsQueryProductsNodes;

    fn extract_nodes(response: Response<Self::ResponseData>) -> DxResult<Vec<Self::Nodes>> {
        let nodes = response
            .data
            .expect("No data received")
            .products
            .expect("No products received");

        tracing::info!("Products: {:?}", nodes);

        let nodes = nodes.nodes;

        Ok(nodes)
    }

    fn from_nodes(nodes: Vec<Self::Nodes>) -> Self {
        Self::from(nodes)
    }

    fn create_variables(first: i64, after: Option<String>) -> Self::Variables {
        Variables { first, after }
    }
}

// Implement Controller trait for Products
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
        sort_direction: Option<PageSort>,
    ) -> DxResult<Self> {
        info!("Fetching products using GraphQLEntity trait");
        Self::make_request(page_size, sort_direction).await
    }
}
