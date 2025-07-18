use async_trait::async_trait;

// Modules
use crate::{
    app::error::{AppError, AppErrorKind, GraphQLErrorWrapper},
    controllers::entity::EntityController,
    graphql::{
        client::GraphQLClient,
        models::category::{
            product_category, product_categories,
            ProductCategory as ProductCategoryGraphQLQuery, 
            ProductCategories as ProductCategoriesGraphQLQuery,
        },
    },
    models::category::{
        ProductCategories, ProductCategory,
    },
};

/// Category controller
#[derive(Debug, Clone)]
pub struct CategoryController {
    /// The GraphQL client used for API communication
    client: GraphQLClient,
}

impl CategoryController {
    /// Creates a new CategoryController
    pub fn new() -> Self {
        Self {
            client: GraphQLClient::new(),
        }
    }

    /// Get a category with products
    ///
    /// Fetches a single product category by its slug, including its products with pagination.
    /// 
    /// **Arguments**
    ///
    /// * `slug` - The slug of the category to get
    /// * `first_products` - The number of products to get
    /// * `after_products` - The cursor to get the next page of products
    ///
    /// **Returns**
    ///
    /// * `ProductCategory` - The category with products
    pub async fn get_with_products(
        &self,
        slug: &str,
        first_products: Option<i64>,
        after_products: Option<String>,
    ) -> Result<ProductCategory, AppError> {
        // Build the request
        let request = product_category::Variables {
            slug: slug.to_string(),
            first_products,
            after_products,
        };

        // Execute the request
        let request = self
            .client
            .execute_query::<
                product_category::Variables,
                ProductCategoryGraphQLQuery,
                product_category::ResponseData,
            >(request)
            .await
            .map_err(|e_string| {
                AppError::new_with_source(
                    AppErrorKind::Api, 
                    "A network or GraphQL error occurred while fetching the category.".to_string(),
                    Some(format!(
                        "Failed to fetch category data for slug '{}'.",
                        slug
                    )),
                    GraphQLErrorWrapper(e_string),
                )
            })?;

        // Cast the category
        let request: Option<ProductCategory> = request.product_category.map(ProductCategory::from);

        // Return the category with products
        request.ok_or_else(|| {
            AppError::new(
                AppErrorKind::NotFound,
                "The requested category could not be found.".to_string(),
                Some("Category not found in GraphQL response.".to_string()),
                None,
            )
        })
    }
}

/// Category EntityController implementation
#[async_trait(?Send)]
impl EntityController for CategoryController {
    /// A single category entity
    type Entity = ProductCategory;
    /// A collection of categories
    type EntityCollection = ProductCategories;


    /// Get a category by slug
    ///
    /// This implementation fetches the category with no products by default.  
    /// To fetch products with pagination, use `get_with_products`.  
    /// 
    /// **Arguments**  
    ///
    /// * `slug` - The slug of the category to get
    ///
    /// **Returns**  
    ///
    /// * `Self::Entity` - The category entity
    async fn get_by_slug(&self, slug: &str) -> Result<Self::Entity, AppError> {
        self.get_with_products(slug, Some(0), None).await
    }

    /// Get a list of categories
    ///
    /// **Arguments**
    ///
    /// * `page_size` - The number of categories to get
    /// * `after` - The cursor to get the next page of categories
    ///
    /// **Returns**
    ///
    /// * `Self::EntityCollection` - The collection of categories
    async fn get_list(
        &self,
        page_size: Option<usize>,
        after: Option<String>,
    ) -> Result<Self::EntityCollection, AppError> {
        // Build the request
        let request = product_categories::Variables {
            first: Some(page_size.unwrap_or(10) as i64),
            after: after.clone(),
        };

        // Execute the request
        let request = self
            .client
            .execute_query::<_, ProductCategoriesGraphQLQuery, product_categories::ResponseData>(request)
            .await
            .map_err(|e_string| {
                AppError::new_with_source(
                    AppErrorKind::Api, 
                    "A network or GraphQL error occurred while fetching categories.".to_string(),
                    Some(format!(
                        "Failed to fetch product categories list. Page size: {:?}, After: {:?}.",
                        page_size, after
                    )),
                    GraphQLErrorWrapper(e_string),
                )
            })?;

        // Cast the categories
        let request: Option<ProductCategories> = request.product_categories.map(ProductCategories::from);

        // Return the categories
        request.ok_or_else(|| {
            AppError::new(
                AppErrorKind::NotFound,
                "The requested categories could not be found.".to_string(),
                Some("Categories not found in GraphQL response.".to_string()),
                None,
            )
        })
    }
}
