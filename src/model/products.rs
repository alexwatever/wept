use graphql_client::GraphQLQuery;
use parse_display::FromStr;
use products_query::ProductsQueryProductsNodes;
use serde::{Deserialize, Serialize};
use std::fmt::{Display as FmtDisplay, Formatter, Result as FmtResult};

/// # Product
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub(crate) struct Product {
    pub(crate) id: String,
    pub(crate) sku: Option<String>,
    pub(crate) slug: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) status: Option<String>,
    pub(crate) description: Option<String>,
}

/// # Products
#[derive(Debug)]
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
    fn from(product: Vec<ProductsQueryProductsNodes>) -> Self {
        Self(
            product
                .into_iter()
                .map(|product: ProductsQueryProductsNodes| product.into())
                .collect(),
        )
    }
}

impl From<ProductsQueryProductsNodes> for Product {
    /// # Convert a `ProductsQueryProductsNodes` to a `Product`
    fn from(product: ProductsQueryProductsNodes) -> Self {
        Self {
            id: product.id,
            sku: product.sku,
            slug: product.slug,
            name: product.name,
            status: product.status,
            description: product.description,
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

/// # Product Size
#[derive(Default, FromStr, Debug)]
#[display(style = "snake_case")]
pub(crate) enum ProductSize {
    Small,
    #[default]
    Medium,
    Large,
}
