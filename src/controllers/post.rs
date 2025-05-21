use async_trait::async_trait;

// Modules
use crate::{
    app::error::{AppError, AppErrorKind, GraphQLErrorWrapper},
    controllers::common::EntityController,
    graphql::{
        client::GraphQLClient,
        models::post::{post_query, posts_query, PostQuery, PostsQuery},
    },
    models::post::{Post, Posts},
};

/// Post controller
#[derive(Debug)]
pub struct PostController {
    /// The GraphQL client used for API communication
    client: GraphQLClient,
}

impl PostController {
    /// Creates a new post controller
    pub fn new() -> Self {
        Self {
            client: GraphQLClient::new(),
        }
    }
}

/// Post controller implementation
#[async_trait(?Send)]
impl EntityController for PostController {
    /// A single post entity
    type Entity = Post;
    /// A collection of posts
    type EntityCollection = Posts;

    /// Get a post by slug
    ///
    /// **Arguments**
    ///
    /// * `slug` - The slug of the post to get
    ///
    /// **Returns**
    ///
    /// * `Self::Entity` - The post entity
    async fn get_by_slug(&self, slug: &str) -> Result<Self::Entity, AppError> {
        // Build the request
        let request: post_query::Variables = post_query::Variables {
            slug: slug.to_string(),
        };
        let request = self
            .client
            .execute_query::<_, PostQuery, post_query::ResponseData>(request);

        // Execute the request
        let request: post_query::ResponseData = request.await.map_err(|err| {
            AppError::new_with_source(
                AppErrorKind::GraphQL,
                "An error occurred while fetching the post.".to_string(),
                Some(format!(
                    "Failed to execute get_post query for slug '{slug}'"
                )),
                GraphQLErrorWrapper(err),
            )
        })?;

        // Cast the post
        let request: Option<Post> = request.post.map(Post::from);

        // Return the post
        request.ok_or_else(|| {
            AppError::new(
                AppErrorKind::NotFound,
                "The requested post could not be found.".to_string(),
                Some(format!(
                    "Post with slug '{slug}' not found in GraphQL response."
                )),
                None,
            )
        })
    }

    /// Get multiple posts
    ///
    /// **Arguments**
    ///
    /// * `page_size` - The number of posts to get
    /// * `after` - The cursor to get the next page of posts
    ///
    /// **Returns**
    ///
    /// * `Self::EntityCollection` - The collection of posts
    async fn get_list(
        &self,
        page_size: Option<usize>,
        after: Option<String>,
    ) -> Result<Self::EntityCollection, AppError> {
        // Build the request
        let request = posts_query::Variables {
            first: Some(page_size.unwrap_or(10) as i64),
            after: after.clone(),
        };
        let request = self
            .client
            .execute_query::<_, PostsQuery, posts_query::ResponseData>(request);

        // Execute the request
        let request: posts_query::ResponseData = request.await.map_err(|err| {
            AppError::new_with_source(
                AppErrorKind::GraphQL,
                "An error occurred while fetching the list of posts.".to_string(),
                Some(format!(
                    "Failed to execute get_posts query. Page size: '{page_size:?}', After: '{after:?}'"
                )),
                GraphQLErrorWrapper(err),
            )
        })?;

        // Cast the posts
        let request: Option<Posts> = request.posts.map(Posts::from);

        // Return the posts
        request.ok_or_else(|| {
            AppError::new(
                AppErrorKind::NotFound,
                "The requested posts could not be found.".to_string(),
                Some("Posts not found in GraphQL response.".to_string()),
                None,
            )
        })
    }
}
