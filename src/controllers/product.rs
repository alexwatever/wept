use async_trait::async_trait;

// Modules
use crate::{
    app::error::{AppError, AppErrorKind, GraphQLErrorWrapper},
    controllers::common::EntityController,
    graphql::{
        client::GraphQLClient,
        models::product::{
            product_query, products_query, search_products_query, ProductQuery, ProductsQuery,
            SearchProductsQuery,
        },
    },
    models::product::{Product, Products},
};

/// Product controller
#[derive(Debug, Clone)]
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

    /// Search for products by a given search term.
    pub async fn search_products(&self, search_term: &str) -> Result<Products, AppError> {
        let variables = search_products_query::Variables {
            search: search_term.to_string(),
        };

        let response_body = self
            .client
            .execute_query::<_, SearchProductsQuery, search_products_query::ResponseData>(variables)
            .await
            .map_err(|err| {
                AppError::new_with_source(
                    AppErrorKind::GraphQL,
                    "An error occurred while searching for products.".to_string(),
                    Some(format!(
                        "Failed to execute search_products query for term '{search_term}'"
                    )),
                    GraphQLErrorWrapper(err),
                )
            })?;

        let products = response_body.products.map(Products::from).ok_or_else(|| {
            AppError::new(
                AppErrorKind::NotFound,
                "No products found matching the search term.".to_string(),
                Some(format!("No products found for search term '{search_term}'")),
                None,
            )
        })?;

        Ok(products)
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
