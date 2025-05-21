use serde::{Deserialize, Serialize};

// Modules
use super::{
    pagination::Pagination,
    product::{Product, Products},
};
use crate::graphql::models::category::{
    product_categories::{
        ProductCategoriesProductCategories, ProductCategoriesProductCategoriesEdges,
        ProductCategoriesProductCategoriesEdgesNode,
        ProductCategoriesProductCategoriesEdgesNodeImage,
        ProductCategoriesProductCategoriesPageInfo,
    },
    product_category::{
        ProductCategoryProductCategory, ProductCategoryProductCategoryImage,
        ProductCategoryProductCategoryProducts, ProductCategoryProductCategoryProductsPageInfo,
    },
};

/// Represents a WooCommerce product category
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ProductCategory {
    /// Category ID (GraphQL global ID)
    pub id: String,
    /// Category database ID
    pub database_id: i64,
    /// Category name
    pub name: Option<String>,
    /// Category slug
    pub slug: Option<String>,
    /// Category description
    pub description: Option<String>,
    /// Number of products in the category
    pub count: Option<i64>,
    /// Category image
    pub image: Option<ProductCategoryImage>,
    /// Products in the category
    pub products: Option<Products>,
    /// Pagination info for products in the category
    pub page_info: Option<Pagination>,
}

impl From<ProductCategoryProductCategory> for ProductCategory {
    /// Convert a GraphQL product category to a ProductCategory model
    ///
    /// **Arguments**
    ///
    /// * `gql_category` - The GraphQL product category to convert
    ///
    /// **Returns**
    ///
    /// * `Self` - The converted ProductCategory model
    fn from(category: ProductCategoryProductCategory) -> Self {
        let image: Option<ProductCategoryImage> = category.image.map(ProductCategoryImage::from);

        let products_struct_option: Option<Products> = category.products.map(
            |gql_category_products: ProductCategoryProductCategoryProducts| {
                let products_vec: Vec<Product> = gql_category_products
                    .edges
                    .into_iter()
                    .map(|edge| Product::from(edge.node))
                    .collect();
                let page_info_for_products_struct =
                    Some(Pagination::from(gql_category_products.page_info));
                Products {
                    products: products_vec,
                    page_info: page_info_for_products_struct,
                }
            },
        );

        Self {
            id: category.id,
            database_id: category.database_id,
            name: category.name,
            slug: category.slug,
            description: category.description,
            count: category.count,
            image,
            page_info: products_struct_option
                .as_ref()
                .and_then(|p| p.page_info.clone()),
            products: products_struct_option,
        }
    }
}

impl From<ProductCategoriesProductCategoriesEdgesNode> for ProductCategory {
    /// Convert a GraphQL product category node to a ProductCategory model
    ///
    /// **Arguments**
    ///
    /// * `node` - The GraphQL product category node to convert
    ///
    /// **Returns**
    ///
    /// * `Self` - The converted ProductCategory model
    fn from(category: ProductCategoriesProductCategoriesEdgesNode) -> Self {
        Self {
            id: category.id,
            database_id: category.database_id,
            name: category.name,
            slug: category.slug,
            description: category.description,
            count: category.count,
            image: category.image.map(ProductCategoryImage::from),
            products: None,
            page_info: None,
        }
    }
}

/// Represents a collection of product categories.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Default)]
pub struct ProductCategories {
    pub categories: Vec<ProductCategory>,
    pub page_info: Option<Pagination>,
}

impl From<ProductCategoriesProductCategories> for ProductCategories {
    /// Convert GraphQL product categories to a ProductCategories model
    ///
    /// **Arguments**
    ///
    /// * `categories` - The GraphQL product categories to convert
    ///
    /// **Returns**
    ///
    /// * `Self` - The converted ProductCategories model
    fn from(categories_gql: ProductCategoriesProductCategories) -> Self {
        // Convert the GraphQL specific PageInfo to our common Pagination model
        let page_info_converted: Option<Pagination> =
            Some(Pagination::from(categories_gql.page_info));

        let categories_vec: Vec<ProductCategory> = categories_gql
            .edges
            .into_iter()
            .map(|category_edge: ProductCategoriesProductCategoriesEdges| {
                ProductCategory::from(category_edge.node)
            })
            .collect();

        Self {
            categories: categories_vec,
            page_info: page_info_converted,
        }
    }
}

impl From<ProductCategoryProductCategoryProductsPageInfo> for Pagination {
    /// Convert a GraphQL product category products pagination to a PageInfo model
    ///
    /// **Arguments**
    ///
    /// * `page_info` - The GraphQL product category products pagination to convert
    ///
    /// **Returns**
    ///
    /// * `Self` - The converted PageInfo model
    fn from(page_info: ProductCategoryProductCategoryProductsPageInfo) -> Self {
        Self {
            end_cursor: page_info.end_cursor,
            has_next_page: page_info.has_next_page,
        }
    }
}

impl From<ProductCategoriesProductCategoriesPageInfo> for Pagination {
    /// Convert a GraphQL product categories pagination to a PageInfo model
    ///
    /// **Arguments**
    ///
    /// * `page_info` - The GraphQL product categories pagination to convert
    ///
    /// **Returns**
    ///
    /// * `Self` - The converted PageInfo model
    fn from(page_info: ProductCategoriesProductCategoriesPageInfo) -> Self {
        Self {
            end_cursor: page_info.end_cursor,
            has_next_page: page_info.has_next_page,
        }
    }
}

/// Represents an image associated with a product category
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ProductCategoryImage {
    /// Image ID
    pub id: Option<String>,
    /// Image URL
    pub source_url: Option<String>,
    /// Image alt text
    pub alt_text: Option<String>,
}

impl From<ProductCategoryProductCategoryImage> for ProductCategoryImage {
    /// Convert a GraphQL product category image to a ProductCategoryImage model
    ///
    /// **Arguments**
    ///
    /// * `image` - The GraphQL product category image to convert
    ///
    /// **Returns**
    ///
    /// * `Self` - The converted ProductCategoryImage model
    fn from(image: ProductCategoryProductCategoryImage) -> Self {
        Self {
            id: Some(image.id),
            source_url: image.source_url,
            alt_text: image.alt_text,
        }
    }
}

impl From<ProductCategoriesProductCategoriesEdgesNodeImage> for ProductCategoryImage {
    /// Convert a GraphQL product category image node to a ProductCategoryImage model
    ///
    /// **Arguments**
    ///
    /// * `image` - The GraphQL product category image node to convert
    ///
    /// **Returns**
    ///
    /// * `Self` - The converted ProductCategoryImage model
    fn from(image: ProductCategoriesProductCategoriesEdgesNodeImage) -> Self {
        Self {
            id: Some(image.id),
            source_url: image.source_url,
            alt_text: image.alt_text,
        }
    }
}
