use dioxus::prelude::*;
use graphql_client::GraphQLQuery;
use parse_display::Display;
use products_query::{ProductsQueryProductsNodes, ProductsQueryProductsNodesOn};
use serde::{Deserialize, Serialize};
use std::fmt::{Display as FmtDisplay, Formatter, Result as FmtResult};

use crate::view::components::entity_list::EntityDisplay;

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
        // Process image if available
        // let image: Option<ProductImage> = product.image.map(|img| ProductImage {
        //     id: Some(img.id),
        //     source_url: img.source_url,
        //     alt_text: img.alt_text,
        //     title: img.title,
        // });

        // // Process gallery images if available
        // let gallery_images = product.gallery_images.and_then(|gallery| {
        //     gallery.nodes.map(|nodes| {
        //         nodes
        //             .into_iter()
        //             .map(|img| ProductImage {
        //                 id: Some(img.id),
        //                 source_url: img.source_url,
        //                 alt_text: img.alt_text,
        //                 title: None,
        //             })
        //             .collect()
        //     })
        // });

        // Tests
        let simple_product: ProductsQueryProductsNodesOn = product.on;
        match simple_product {
            ProductsQueryProductsNodesOn::SimpleProduct(..) => {
                tracing::info!("Simple product: {:?}", simple_product);
            }
            ProductsQueryProductsNodesOn::ExternalProduct => {
                tracing::info!("External product: {:?}", simple_product);
            }
            ProductsQueryProductsNodesOn::GroupProduct => {
                tracing::info!("Group product: {:?}", simple_product);
            }
            ProductsQueryProductsNodesOn::SimpleProductVariation => {
                tracing::info!("Simple product variation: {:?}", simple_product);
            }
            ProductsQueryProductsNodesOn::VariableProduct => {
                tracing::info!("Variable product: {:?}", simple_product);
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

            // Temp
            image: None,
            gallery_images: None,
            simple_product: Some(SimpleProduct {
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
            }),
            // image,
            // gallery_images,

            // SimpleProduct fields
            // on_sale: product.on_sale,
            // stock_status: product.stock_status,
            // price: product.price,
            // raw_price: product.raw_price,
            // regular_price: product.as_ref().and_then(|s| s.regular_price.clone()),
            // sale_price: simple_product.as_ref().and_then(|s| s.sale_price.clone()),
            // stock_quantity: simple_product.as_ref().and_then(|s| s.stock_quantity),
            // sold_individually: simple_product.as_ref().map(|s| s.sold_individually),
            // review_count: simple_product.as_ref().and_then(|s| s.review_count),
            // weight: simple_product.as_ref().and_then(|s| s.weight.clone()),
            // length: simple_product.as_ref().and_then(|s| s.length.clone()),
            // width: simple_product.as_ref().and_then(|s| s.width.clone()),
            // height: simple_product.as_ref().and_then(|s| s.height.clone()),
            // purchasable: simple_product.as_ref().map(|s| s.purchasable),
            // virtual_product: simple_product.as_ref().map(|s| s.virtual_product),
            // downloadable: simple_product.as_ref().map(|s| s.downloadable),
            // download_limit: simple_product.as_ref().and_then(|s| s.download_limit),
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

impl EntityDisplay for Product {
    fn render(&self) -> Element {
        let Product {
            id,
            name,
            description,
            image,
            simple_product,
            ..
        } = self.clone();

        let price = simple_product.as_ref().and_then(|s| s.price.clone());

        rsx! {
            section { class: "h-40 p-2 m-2 shadow-lg ring-1 rounded-lg flex flex-row place-items-center hover:ring-4 hover:shadow-2xl transition-all duration-200",
                // Display product image if available
                {
                    if let Some(img) = image {
                        rsx! {
                            img {
                                class: "object-scale-down w-1/6 h-full",
                                src: "{img.source_url.clone().unwrap_or_default()}",
                                alt: "{img.alt_text.clone().unwrap_or_default()}"
                            }
                        }
                    } else {
                        rsx! {
                            div { class: "w-1/6 h-full bg-gray-200 flex items-center justify-center",
                                "No Image"
                            }
                        }
                    }
                }
                div { class: "pl-4 text-left text-ellipsis",
                    a {
                        href: "/products/{id}",
                        class: "w-full text-center font-bold text-xl",
                        "{name.clone().unwrap_or_default()}"
                    }
                    {
                        if let Some(price_str) = price {
                            rsx! {
                                p {
                                    class: "w-full text-sm font-bold text-green-600",
                                    "{price_str}"
                                }
                            }
                        } else {
                            rsx! { "" }
                        }
                    }
                    p {
                        class: "w-full text-sm overflow-hidden line-clamp-3",
                        "{description.clone().unwrap_or_default()}"
                    }
                }
            }
        }
    }
}
