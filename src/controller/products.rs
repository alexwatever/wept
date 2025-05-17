use std::io::ErrorKind;

use dioxus::Result as DxResult;
use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;
use tracing::info;

// # Modules
use super::{base::GraphQLEntity, Controller};
use crate::{
    model::{
        pagination::PageSort,
        products::{
            product_query,
            products_query::{ProductsQueryProductsNodes, ResponseData, Variables},
            Product, ProductImage, ProductQuery, Products, ProductsQuery, SimpleProduct,
        },
    },
    State,
};

/// # Fetch a single product by slug
///
/// Get a product from the WordPress GraphQL API by its slug.
///
/// **Arguments**
/// * `slug` - The slug of the product to fetch.
///
/// **Returns**
/// A product.
pub(crate) async fn fetch_product(slug: String) -> DxResult<Product> {
    // Create variables for the GraphQL query
    let variables = product_query::Variables { slug };

    // Build the GraphQL query
    let payload = ProductQuery::build_query(variables);

    // Build the endpoint
    let endpoint = format!(
        "{host}/{path}",
        host = State::get_backend_host(),
        path = State::get_backend_path()
    );

    // Make the request to the GraphQL API
    let client = Client::new();
    let response = client.post(endpoint).json(&payload).send().await?;

    // Parse the response
    let response_data: Response<product_query::ResponseData> = response.json().await?;

    // Check if we have errors
    if let Some(errors) = response_data.errors {
        if !errors.is_empty() {
            let error_msg = errors
                .iter()
                .map(|e| e.message.clone())
                .collect::<Vec<String>>()
                .join(", ");

            // Use an error type that implements StdError
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("GraphQL error: {}", error_msg),
            )
            .into());
        }
    }

    // Extract the product data
    let data = response_data
        .data
        .ok_or_else(|| std::io::Error::new(ErrorKind::NotFound, "No data received"))?;

    let product_data = data
        .product
        .ok_or_else(|| std::io::Error::new(ErrorKind::NotFound, "Product not found"))?;

    // Convert the product data to a Product model
    let product = convert_product_from_query(product_data)?;

    Ok(product)
}

/// Convert product data from GraphQL query to Product model
fn convert_product_from_query(product: product_query::ProductQueryProduct) -> DxResult<Product> {
    // Initialize a SimpleProduct with default/empty values
    let mut simple_product_data = SimpleProduct {
        on_sale: None,
        stock_status: None,
        price: None,
        raw_price: None,
        regular_price: None,
        sale_price: None,
        stock_quantity: None,
        sold_individually: None,
        review_count: None,
        weight: None,
        length: None,
        width: None,
        height: None,
        purchasable: None,
        virtual_product: None,
        downloadable: None,
        download_limit: None,
    };

    // Extract data based on product type
    // Directly assign product.on to a variable
    let on = product.on;

    // Match on the product type
    match on {
        product_query::ProductQueryProductOn::SimpleProduct(simple_product) => {
            simple_product_data = SimpleProduct {
                on_sale: simple_product.on_sale,
                stock_status: simple_product.stock_status.map(|s| format!("{:?}", s)),
                price: simple_product.price.clone(),
                raw_price: simple_product.raw_price.clone(),
                regular_price: simple_product.regular_price.clone(),
                sale_price: simple_product.sale_price.clone(),
                stock_quantity: simple_product.stock_quantity.map(|q| q as i32),
                sold_individually: simple_product.sold_individually,
                review_count: simple_product.review_count.map(|c| c as i32),
                weight: simple_product.weight.clone(),
                length: simple_product.length.clone(),
                width: simple_product.width.clone(),
                height: simple_product.height.clone(),
                purchasable: simple_product.purchasable,
                virtual_product: simple_product.virtual_,
                downloadable: simple_product.downloadable,
                download_limit: simple_product.download_limit.map(|l| l as i32),
            };
        }
        product_query::ProductQueryProductOn::VariableProduct(variable_product) => {
            info!(
                "Variable product with status: {:?}",
                variable_product.stock_status
            );
            // Could populate additional fields relevant to variable products here
            simple_product_data.on_sale = variable_product.on_sale;
            simple_product_data.stock_status =
                variable_product.stock_status.map(|s| format!("{:?}", s));
        }
        product_query::ProductQueryProductOn::ExternalProduct(external_product) => {
            info!(
                "External product with URL: {:?}",
                external_product.external_url
            );
            simple_product_data.on_sale = external_product.on_sale;
        }
        product_query::ProductQueryProductOn::GroupProduct(_) => {
            info!("Group product");
            // Group products have specific functionality
        }
        product_query::ProductQueryProductOn::SimpleProductVariation => {
            info!("Simple product variation");
            // Handle simple product variation
        }
    }

    // Process gallery images if available
    let gallery_images = product.gallery_images.map(|gallery| {
        gallery
            .nodes
            .into_iter()
            .map(|img| ProductImage {
                id: Some(img.id),
                source_url: img.source_url,
                alt_text: img.alt_text,
                title: None,
            })
            .collect::<Vec<ProductImage>>()
    });

    // Create the product model with all the data
    let product = Product {
        id: product.id,
        sku: product.sku,
        slug: product.slug,
        name: product.name,
        status: product.status,
        description: product.description,
        short_description: product.short_description,
        date_on_sale_from: product.date_on_sale_from,
        date_on_sale_to: product.date_on_sale_to,
        featured_image_id: product.featured_image_id,
        image: product.image.map(|img| ProductImage {
            id: Some(img.id),
            source_url: img.source_url,
            alt_text: img.alt_text,
            title: img.title,
        }),
        gallery_images,
        simple_product: Some(simple_product_data),
    };

    Ok(product)
}

// Removed placeholder fetch_products function -
// The proper GraphQL implementation is provided through the Controller trait

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
        Self::make_request(page_size, sort_direction).await
    }
}
