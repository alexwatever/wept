use serde::{Deserialize, Serialize};
use tracing::warn;

// Modules
use crate::{
    graphql::models::{
        category::product_category::{
            ProductCategoryProductCategoryProductsEdgesNode as ProductCategoryGraphqlProductNode,
            ProductCategoryProductCategoryProductsEdgesNodeImage as ProductCategoryGraphqlProductImage,
            ProductCategoryProductCategoryProductsEdgesNodeOn as ProductCategoryGraphqlProductNodeOn,
        },
        product::{
            product_query::{
                ProductQueryProduct, ProductQueryProductGalleryImages,
                ProductQueryProductGalleryImagesNodes, ProductQueryProductOn,
                ProductQueryProductOnSimpleProduct,
            },
            products_query::{
                ProductsQueryProducts, ProductsQueryProductsNodes,
                ProductsQueryProductsNodesGalleryImages,
                ProductsQueryProductsNodesGalleryImagesNodes, ProductsQueryProductsNodesImage,
                ProductsQueryProductsNodesOn, ProductsQueryProductsPageInfo,
            },
            search_products_query,
        },
    },
    models::pagination::Pagination,
    views::components::common::entity_list::EntityList,
};

/// Product entity representing a WooCommerce product
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Default)]
pub struct Product {
    /// Product ID
    pub id: String,
    /// Product SKU
    pub sku: Option<String>,
    /// Product slug
    pub slug: Option<String>,
    /// Product name
    pub name: Option<String>,
    /// Product status
    pub status: Option<String>,
    /// Product description
    pub description: Option<String>,
    /// Product short description
    pub short_description: Option<String>,
    /// Date product goes on sale
    pub date_on_sale_from: Option<String>,
    /// Date product sale ends
    pub date_on_sale_to: Option<String>,
    /// Featured image ID
    pub featured_image_id: Option<String>,
    /// Main product image
    pub image: Option<ProductImage>,
    /// Product gallery images
    pub gallery_images: Option<Vec<ProductImage>>,
    /// Simple product data
    pub simple_product: Option<ProductQueryProductOnSimpleProduct>,
    /// Product database ID
    pub database_id: Option<i64>,
}

impl From<ProductQueryProduct> for Product {
    fn from(product: ProductQueryProduct) -> Self {
        let (database_id, simple_product_data) = match product.on {
            ProductQueryProductOn::SimpleProduct(sp) => (Some(sp.database_id), Some(sp)),
            _ => (None, None),
        };

        let image = product.image.map(|image| ProductImage {
            id: Some(image.id),
            source_url: image.source_url,
            alt_text: image.alt_text,
            title: image.title,
        });
        let featured_image_id = image.as_ref().and_then(|img| img.id.clone());
        let gallery_images = product.gallery_images.map(|images| images.into());

        Self {
            id: product.id,
            database_id,
            sku: product.sku,
            slug: product.slug,
            name: product.name,
            status: product.status,
            description: product.description,
            short_description: product.short_description,
            date_on_sale_from: product.date_on_sale_from,
            date_on_sale_to: product.date_on_sale_to,
            featured_image_id,
            image,
            gallery_images,
            simple_product: simple_product_data,
        }
    }
}

/// Collection of products with pagination information
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct Products {
    pub products: Vec<Product>,
    pub page_info: Option<Pagination>,
}

impl EntityList for Products {
    type Entity = Product;
    fn as_slice(&self) -> &[Self::Entity] {
        &self.products
    }
}

impl From<ProductsQueryProducts> for Products {
    /// Convert a ProductsQueryProducts to a Products
    ///
    /// **Arguments**
    ///
    /// * `products` - The GraphQL products to convert
    ///
    /// **Returns**
    ///
    /// * `Self` - The converted Products
    fn from(products: ProductsQueryProducts) -> Self {
        let page_info: Option<Pagination> = Some(Pagination::from(products.page_info));
        let products: Vec<Product> = products.nodes.into_iter().map(Product::from).collect();

        Self {
            products,
            page_info,
        }
    }
}

impl From<search_products_query::SearchProductsQueryProducts> for Products {
    fn from(products: search_products_query::SearchProductsQueryProducts) -> Self {
        let products: Vec<Product> = products
            .edges
            .into_iter()
            .map(|edge| Product::from(edge.node))
            .collect();

        Self {
            products,
            page_info: None, // Search does not have pagination
        }
    }
}

impl From<ProductsQueryProductsPageInfo> for Pagination {
    /// Convert a ProductsQueryProductsPageInfo to a Pagination
    ///
    /// **Arguments**
    ///
    /// * `page_info` - The GraphQL products page info to convert
    ///
    /// **Returns**
    ///
    /// * `Self` - The converted Pagination
    fn from(page_info: ProductsQueryProductsPageInfo) -> Self {
        Self {
            end_cursor: page_info.end_cursor,
            has_next_page: page_info.has_next_page,
        }
    }
}

/// Simple product data
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Default)]
pub struct SimpleProduct {
    /// Whether the product is on sale
    pub on_sale: Option<bool>,
    /// Stock status
    pub stock_status: Option<String>,
    /// Formatted price
    pub price: Option<String>,
    /// Raw price value
    pub raw_price: Option<String>,
    /// Regular price
    pub regular_price: Option<String>,
    /// Sale price
    pub sale_price: Option<String>,
    /// Stock quantity
    pub stock_quantity: Option<i32>,
    /// Whether the product is sold individually
    pub sold_individually: Option<bool>,
    /// Number of reviews
    pub review_count: Option<i32>,
    /// Product weight
    pub weight: Option<String>,
    /// Product length
    pub length: Option<String>,
    /// Product width
    pub width: Option<String>,
    /// Product height
    pub height: Option<String>,
    /// Whether the product is purchasable
    pub purchasable: Option<bool>,
    /// Whether the product is virtual
    pub virtual_product: Option<bool>,
    /// Whether the product is downloadable
    pub downloadable: Option<bool>,
    /// Download limit
    pub download_limit: Option<i32>,
}

/// Product image
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ProductImage {
    /// Image ID
    pub id: Option<String>,
    /// Image URL
    pub source_url: Option<String>,
    /// Image alt text
    pub alt_text: Option<String>,
    /// Image title
    pub title: Option<String>,
}

impl From<ProductQueryProductGalleryImages> for Vec<ProductImage> {
    /// Convert a ProductQueryProductGalleryImages to a Vec<ProductImage>
    ///
    /// **Arguments**
    ///
    /// * `images` - The GraphQL product gallery images to convert
    ///
    /// **Returns**
    ///
    /// * `Vec<ProductImage>` - The converted ProductImage
    fn from(images: ProductQueryProductGalleryImages) -> Vec<ProductImage> {
        images.nodes.into_iter().map(ProductImage::from).collect()
    }
}

impl From<ProductQueryProductGalleryImagesNodes> for ProductImage {
    /// Convert a ProductQueryProductGalleryImagesNodes to a ProductImage
    ///
    /// **Arguments**
    ///
    /// * `image` - The GraphQL product image to convert
    ///
    /// **Returns**
    ///
    /// * `ProductImage` - The converted ProductImage
    fn from(image: ProductQueryProductGalleryImagesNodes) -> Self {
        Self {
            id: Some(image.id),
            source_url: image.source_url,
            alt_text: image.alt_text,
            title: image.title,
        }
    }
}

impl From<ProductsQueryProductsNodesGalleryImages> for Vec<ProductImage> {
    /// Convert a ProductQueryProductGalleryImages to a Vec<ProductImage>
    ///
    /// **Arguments**
    ///
    /// * `images` - The GraphQL product gallery images to convert
    ///
    /// **Returns**
    ///
    /// * `Vec<ProductImage>` - The converted ProductImage
    fn from(images: ProductsQueryProductsNodesGalleryImages) -> Vec<ProductImage> {
        images.nodes.into_iter().map(ProductImage::from).collect()
    }
}

impl From<ProductsQueryProductsNodesGalleryImagesNodes> for ProductImage {
    /// Convert a ProductsQueryProductsNodesGalleryImagesNodes to a ProductImage
    ///
    /// **Arguments**
    ///
    /// * `image` - The GraphQL product image to convert
    ///
    /// **Returns**
    ///
    /// * `ProductImage` - The converted ProductImage
    fn from(image: ProductsQueryProductsNodesGalleryImagesNodes) -> Self {
        Self {
            id: Some(image.id),
            source_url: image.source_url,
            alt_text: image.alt_text,
            title: image.title,
        }
    }
}

impl From<ProductCategoryGraphqlProductImage> for ProductImage {
    /// Convert a ProductCategoryGraphqlProductImage to a ProductImage
    ///
    /// **Arguments**
    ///
    /// * `image` - The GraphQL product image to convert
    ///
    /// **Returns**
    ///
    /// * `ProductImage` - The converted ProductImage
    fn from(image: ProductCategoryGraphqlProductImage) -> Self {
        Self {
            id: Some(image.id),
            source_url: image.source_url,
            alt_text: image.alt_text,
            title: None,
        }
    }
}

impl From<ProductsQueryProductsNodesImage> for ProductImage {
    /// Convert a ProductsQueryProductsNodesImage to a ProductImage
    ///
    /// **Arguments**
    ///
    /// * `image` - The GraphQL product image to convert
    ///
    /// **Returns**
    ///
    /// * `ProductImage` - The converted ProductImage
    fn from(image: ProductsQueryProductsNodesImage) -> Self {
        Self {
            id: Some(image.id),
            source_url: image.source_url,
            alt_text: image.alt_text,
            title: None,
        }
    }
}

/// From `ProductsQueryProductsNodes` to `Product`
impl From<ProductsQueryProductsNodes> for Product {
    fn from(product: ProductsQueryProductsNodes) -> Self {
        let (database_id, simple_product_data) = match product.on {
            ProductsQueryProductsNodesOn::SimpleProduct(sp) => (Some(sp.database_id), Some(sp)),
            _ => (None, None),
        };

        let image = product.image.map(ProductImage::from);
        let featured_image_id = image.as_ref().and_then(|img| img.id.clone());
        let gallery_images = product.gallery_images.map(|images| images.into());

        Self {
            id: product.id,
            database_id,
            sku: product.sku,
            slug: product.slug,
            name: product.name,
            status: product.status,
            description: product.description,
            short_description: product.short_description,
            date_on_sale_from: product.date_on_sale_from,
            date_on_sale_to: product.date_on_sale_to,
            featured_image_id,
            image,
            gallery_images,
            simple_product: simple_product_data,
        }
    }
}

impl From<ProductCategoryGraphqlProductNode> for Product {
    /// Convert a ProductCategoryGraphqlProductNode to a Product
    ///
    /// **Arguments**
    ///
    /// * `product` - The GraphQL product node to convert
    ///
    /// **Returns**
    ///
    /// * `Product` - The converted Product
    fn from(product: ProductCategoryGraphqlProductNode) -> Self {
        // Get the simple product details
        let simple_product: Option<SimpleProduct> = match product.on {
            ProductCategoryGraphqlProductNodeOn::SimpleProduct(sp) => Some(SimpleProduct {
                price: sp.price.clone(),
                raw_price: sp.raw_price.clone(),
                regular_price: sp.regular_price.clone(),
                sale_price: sp.sale_price.clone(),
                stock_status: sp.stock_status.map(|se| format!("{:?}", se)),
                on_sale: None,
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
            }),
            _ => {
                warn!("Product node an unsupported type when converting from ProductCategoryGraphqlProductNode: {:?}", product.on);
                None
            }
        };

        // Get image
        let image: Option<ProductImage> = product.image.map(ProductImage::from);
        let featured_image_id: Option<String> = image.as_ref().and_then(|img_m| img_m.id.clone());

        Self {
            id: product.id,
            slug: product.slug,
            name: product.name,
            image,
            featured_image_id,
            sku: None,
            status: None,
            description: None,
            short_description: None,
            date_on_sale_from: None,
            date_on_sale_to: None,
            gallery_images: None,
            simple_product,
            database_id: None,
        }
    }
}

/// From `SearchProductsQueryProductsEdgesNode` to `Product`
impl From<search_products_query::SearchProductsQueryProductsEdgesNode> for Product {
    fn from(product: search_products_query::SearchProductsQueryProductsEdgesNode) -> Self {
        let (database_id, simple_product_data) = match product.on {
            search_products_query::SearchProductsQueryProductsEdgesNodeOn::SimpleProduct(sp) => {
                (Some(product.database_id), Some(sp))
            }
            _ => (Some(product.database_id), None),
        };

        let image = product.image.map(|image| ProductImage {
            id: Some(image.id),
            source_url: image.source_url,
            alt_text: image.alt_text,
            title: image.title,
        });
        let featured_image_id = image.as_ref().and_then(|img| img.id.clone());

        Self {
            id: product.id,
            database_id,
            slug: product.slug,
            name: product.name,
            image,
            featured_image_id,
            simple_product: simple_product_data,
            ..Default::default()
        }
    }
}
