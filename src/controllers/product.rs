use async_trait::async_trait;

// Modules
use crate::{
    app::error::{AppError, AppErrorKind, GraphQLErrorWrapper},
    controllers::common::EntityController,
    graphql::{
        client::GraphQLClient,
        models::product::{product_query, products_query, ProductQuery, ProductsQuery},
    },
    models::product::{Product, Products},
};

/// Product controller
#[derive(Debug)]
pub struct ProductController {
    /// The GraphQL client used for API communication
    client: GraphQLClient,
}

impl ProductController {
    /// Creates a new product controller
    pub fn new() -> Self {
        Self {
            client: GraphQLClient::new(),
        }
    }
}

/// Product controller implementation
#[async_trait(?Send)]
impl EntityController for ProductController {
    /// A single product entity
    type Entity = Product;
    /// A collection of products
    type EntityCollection = Products;

    /// Get a product by slug
    ///
    /// **Arguments**
    ///
    /// * `slug` - The slug of the product to get
    ///
    /// **Returns**
    ///
    /// * `Self::Entity` - The product entity
    async fn get_by_slug(&self, slug: &str) -> Result<Self::Entity, AppError> {
        // Build the request
        let request = product_query::Variables {
            slug: slug.to_string(),
        };
        let request = self
            .client
            .execute_query::<_, ProductQuery, product_query::ResponseData>(request);

        // Execute the request
        let request: product_query::ResponseData = request.await.map_err(|err| {
            AppError::new_with_source(
                AppErrorKind::GraphQL,
                "An error occurred while fetching the product.".to_string(),
                Some(format!(
                    "Failed to execute get_product query for slug '{slug}'"
                )),
                GraphQLErrorWrapper(err),
            )
        })?;

        // Cast the product
        let request: Option<Product> = request.product.map(Product::from);

        // Return the product
        request.ok_or_else(|| {
            AppError::new(
                AppErrorKind::NotFound,
                "The requested product could not be found.".to_string(),
                Some(format!(
                    "Product with slug '{slug}' not found in GraphQL response."
                )),
                None,
            )
        })
    }

    /// Get a list of products
    ///
    /// **Arguments**
    ///
    /// * `page_size` - The number of products to get
    /// * `after` - The cursor to get the next page of products
    ///
    /// **Returns**
    ///
    /// * `Self::EntityCollection` - The collection of products
    async fn get_list(
        &self,
        page_size: Option<usize>,
        after: Option<String>,
    ) -> Result<Self::EntityCollection, AppError> {
        // Build the request
        let request = products_query::Variables {
            first: Some(page_size.unwrap_or(10) as i64),
            after: after.clone(),
        };
        let request = self
            .client
            .execute_query::<_, ProductsQuery, products_query::ResponseData>(request);

        // Execute the request
        let request: products_query::ResponseData = request.await.map_err(|err| {
            AppError::new_with_source(
                AppErrorKind::GraphQL,
                "An error occurred while fetching the list of products.".to_string(),
                Some(format!("Failed to execute get_products query. Page size: '{page_size:?}', After: '{after:?}'")),
                GraphQLErrorWrapper(err),
            )
        })?;

        // Cast the products
        let request: Option<Products> = request.products.map(Products::from);

        // Return the products
        request.ok_or_else(|| {
            AppError::new(
                AppErrorKind::NotFound,
                "The requested products could not be found.".to_string(),
                Some("Products not found in GraphQL response.".to_string()),
                None,
            )
        })
    }
}
