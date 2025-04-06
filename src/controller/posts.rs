use dioxus::Result as DxResult;
use graphql_client::{GraphQLQuery, QueryBody, Response};
use reqwest::Client;

// # Modules
use super::Controller;
use crate::{
    model::{
        pagination::PageSort,
        posts::{posts_query, Post, Posts, PostsQuery},
    },
    State,
};

impl Controller for Posts {
    type ReturnedEntity = Posts;

    /// # Fetch a page of posts
    ///
    /// Get a list of posts from the WordPress GraphQL API.  
    ///
    /// **Arguments**  
    /// * `page_size` - The number of posts to fetch.
    /// * `sort_direction` - The direction to sort the posts.
    ///
    /// **Returns**  
    /// A list of posts.
    async fn get_page(
        page_size: Option<usize>,
        _sort_direction: Option<PageSort>,
    ) -> DxResult<Posts> {
        // Build the payload
        let first = page_size.unwrap_or(Self::PAGE_SIZE) as i64;
        let payload = posts_query::Variables { first, after: None };
        let payload: QueryBody<posts_query::Variables> = PostsQuery::build_query(payload);

        // Build the endpoint
        let endpoint: String = format!(
            "{host}/{path}",
            host = State::get_backend_host(),
            path = State::get_backend_path()
        );

        // Make the request
        let request = Client::new().post(endpoint).json(&payload).send().await?;

        // Parse the response
        let posts: Vec<posts_query::PostsQueryPostsNodes> = request
            // Parse the response
            .json::<Response<posts_query::ResponseData>>()
            // Get the posts
            .await?
            .data
            .expect("No data received")
            .posts
            .expect("No posts received")
            .nodes;

        // Build the posts
        let posts: Posts = posts.into();

        Ok(posts)
    }
}

impl From<Vec<posts_query::PostsQueryPostsNodes>> for Posts {
    /// # Convert Vec<posts_query::PostsQueryPostsNodes> to Posts
    fn from(posts: Vec<posts_query::PostsQueryPostsNodes>) -> Self {
        Self(posts.into_iter().map(Post::from).collect())
    }
}

impl From<posts_query::PostsQueryPostsNodes> for Post {
    /// # Convert posts_query::PostsQueryPostsNodes to Post
    fn from(post: posts_query::PostsQueryPostsNodes) -> Self {
        Self {
            id: post.id,
            title: post.title,
            content: post.content,
            slug: post.slug,
        }
    }
}
