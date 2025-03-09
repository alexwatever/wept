use dioxus::Result as DxResult;
use graphql_client::{GraphQLQuery, Response};
use reqwest::get;
use tracing::info;

// # Modules
use crate::model::{
    pagination::PageSort,
    product::{Product, Products},
};

pub(crate) async fn _fetch_products(count: usize, sort: PageSort) -> DxResult<Vec<Product>> {
    let url: String = format!("https://fakestoreapi.com/products/?sort={sort}&limit={count}");

    let request: Vec<Product> = get(&url).await?.json().await?;

    Ok(request)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/query.graphql",
    response_derives = "Debug"
)]
pub struct PostsQuery;

impl Products {
    /// # Fetch Products
    ///
    /// Get a list of products from the WordPress GraphQL API.  
    ///
    /// **Arguments**  
    ///
    /// * `page_count` - The number of products to fetch.
    /// * `page_sort` - The sort order of the products.
    ///
    /// **Returns**  
    ///
    /// A list of products.
    pub(crate) async fn get(page_count: usize, page_sort: PageSort) -> DxResult<Products> {
        let posts_endpoint = "http://localhost:8080/graphql";

        let payload: posts_query::Variables = posts_query::Variables {
            first: page_count as i64,
            after: None,
        };

        let request_body = PostsQuery::build_query(payload);

        let client = reqwest::Client::new();

        let res = client
            .post(posts_endpoint)
            .json(&request_body)
            .send()
            .await?;

        let response_body: Response<posts_query::ResponseData> = res.json().await?;

        let posts = response_body
            .data
            .expect("No data received")
            .posts
            .expect("No posts received")
            .nodes;

        posts.into_iter().for_each(|post| {
            info!("{:?}", post.id);
            info!("{:?}", post.title);
            info!("{:?}", post.content);
            info!("{:?}", post.slug);
        });

        let dummy_endpoint: String =
            format!("https://fakestoreapi.com/products/?sort={page_sort}&limit={page_count}");
        let products: Vec<Product> = get(&dummy_endpoint).await?.json().await?;

        Ok(Products(products))
    }
}
