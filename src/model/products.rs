use graphql_client::GraphQLQuery;
use parse_display::Display;
use products_query::{ProductsQueryProductsNodes, ProductsQueryProductsNodesOn};
use serde::{Deserialize, Serialize};
use std::fmt::{Display as FmtDisplay, Formatter, Result as FmtResult};

/// # Product Image
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub(crate) struct ProductImage {
    pub(crate) id: Option<String>,
    pub(crate) source_url: Option<String>,
    pub(crate) alt_text: Option<String>,
    pub(crate) title: Option<String>,
}

/// # Product Category
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub(crate) struct ProductCategory {
    pub(crate) id: String,
    pub(crate) name: Option<String>,
    pub(crate) slug: Option<String>,
}

/// # Product Attribute
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub(crate) struct ProductAttribute {
    pub(crate) name: Option<String>,
    pub(crate) value: Option<String>,
}

/// # Related Product
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub(crate) struct RelatedProduct {
    pub(crate) id: String,
    pub(crate) name: Option<String>,
    pub(crate) slug: Option<String>,
}

/// # Stock Status
#[derive(Display, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[display(style = "UPPERCASE")]
pub(crate) enum StockStatus {
    #[display("IN_STOCK")]
    InStock,
    #[display("OUT_OF_STOCK")]
    OutOfStock,
    #[display("ON_BACKORDER")]
    OnBackorder,
}

/// # Product
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub(crate) struct Product {
    /// Base fields
    pub(crate) id: String,
    pub(crate) sku: Option<String>,
    pub(crate) slug: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) status: Option<String>,
    pub(crate) description: Option<String>,
    /// Additional base fields
    pub(crate) short_description: Option<String>,
    pub(crate) date_on_sale_from: Option<String>,
    pub(crate) date_on_sale_to: Option<String>,
    pub(crate) featured_image_id: Option<String>,
    pub(crate) image: Option<ProductImage>,
    pub(crate) gallery_images: Option<Vec<ProductImage>>,
    /// SimpleProduct fields
    pub(crate) simple_product: Option<SimpleProduct>,
}

/// # Simple Product
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub(crate) struct SimpleProduct {
    pub(crate) on_sale: Option<bool>,
    pub(crate) stock_status: Option<String>,
    pub(crate) price: Option<String>,
    pub(crate) raw_price: Option<String>,
    pub(crate) regular_price: Option<String>,
    pub(crate) sale_price: Option<String>,
    pub(crate) stock_quantity: Option<i32>,
    pub(crate) sold_individually: Option<bool>,
    pub(crate) review_count: Option<i32>,
    pub(crate) weight: Option<String>,
    pub(crate) length: Option<String>,
    pub(crate) width: Option<String>,
    pub(crate) height: Option<String>,
    pub(crate) purchasable: Option<bool>,
    pub(crate) virtual_product: Option<bool>,
    pub(crate) downloadable: Option<bool>,
    pub(crate) download_limit: Option<i32>,
}

/// # Products
#[derive(Debug, PartialEq)]
pub(crate) struct Products(pub Vec<Product>);

/// # Products GraphQL Query
#[derive(GraphQLQuery, Debug)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/products_query.graphql",
    response_derives = "Debug, Serialize, PartialEq, Eq"
)]
pub struct ProductsQuery;

/// # Product GraphQL Query
#[derive(GraphQLQuery, Debug)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/product_query.graphql",
    response_derives = "Debug, Serialize, PartialEq, Eq"
)]
pub struct ProductQuery;

impl From<Vec<ProductsQueryProductsNodes>> for Products {
    /// # Convert a vector of `ProductsQueryProductsNodes` to a `Products`
    fn from(products: Vec<ProductsQueryProductsNodes>) -> Self {
        Self(
            products
                .into_iter()
                .map(|product: ProductsQueryProductsNodes| product.into())
                .collect(),
        )
    }
}

impl From<ProductsQueryProductsNodes> for Product {
    /// # Convert a `ProductsQueryProductsNodes` to a `Product`
    fn from(product: ProductsQueryProductsNodes) -> Self {
        // Extract data from the variant types
        let product_on = product.on;
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
        match product_on {
            ProductsQueryProductsNodesOn::SimpleProduct(simple_product) => {
                simple_product_data = SimpleProduct {
                    on_sale: simple_product.on_sale,
                    stock_status: simple_product.stock_status.map(|s| format!("{:?}", s)),
                    price: simple_product.price,
                    raw_price: simple_product.raw_price,
                    regular_price: simple_product.regular_price,
                    sale_price: simple_product.sale_price,
                    stock_quantity: simple_product.stock_quantity.map(|q| q as i32),
                    sold_individually: simple_product.sold_individually,
                    review_count: simple_product.review_count.map(|c| c as i32),
                    weight: simple_product.weight,
                    length: simple_product.length,
                    width: simple_product.width,
                    height: simple_product.height,
                    purchasable: simple_product.purchasable,
                    virtual_product: simple_product.virtual_,
                    downloadable: simple_product.downloadable,
                    download_limit: simple_product.download_limit.map(|l| l as i32),
                };
            }
            ProductsQueryProductsNodesOn::ExternalProduct => {
                tracing::info!("External product: {:?}", product_on);
            }
            ProductsQueryProductsNodesOn::GroupProduct => {
                tracing::info!("Group product: {:?}", product_on);
            }
            ProductsQueryProductsNodesOn::SimpleProductVariation => {
                tracing::info!("Simple product variation: {:?}", product_on);
            }
            ProductsQueryProductsNodesOn::VariableProduct => {
                tracing::info!("Variable product: {:?}", product_on);
            }
        }

        Self {
            // Base fields
            id: product.id,
            sku: product.sku,
            slug: product.slug,
            name: product.name,
            status: product.status,
            description: product.description,
            // Additional base fields
            short_description: product.short_description,
            date_on_sale_from: product.date_on_sale_from,
            date_on_sale_to: product.date_on_sale_to,
            featured_image_id: product.featured_image_id,
            // Process image data
            image: product.image.map(|img| ProductImage {
                id: Some(img.id),
                source_url: img.source_url,
                alt_text: img.alt_text,
                title: img.title,
            }),
            gallery_images: product.gallery_images.map(|gallery| {
                gallery
                    .nodes
                    .into_iter()
                    .map(|img| ProductImage {
                        id: Some(img.id),
                        source_url: img.source_url,
                        alt_text: img.alt_text,
                        title: None,
                    })
                    .collect()
            }),
            // Simple product data
            simple_product: Some(simple_product_data),
        }
    }
}

/// # Product Rating
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub(crate) struct ProductRating {
    pub(crate) rate: f32,
    pub(crate) count: u32,
}

impl FmtDisplay for ProductRating {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let rounded = self.rate.round() as usize;
        for _ in 0..rounded {
            "★".fmt(f)?;
        }
        for _ in 0..(5 - rounded) {
            "☆".fmt(f)?;
        }

        write!(f, " ({:01}) ({} ratings)", self.rate, self.count)?;

        Ok(())
    }
}
