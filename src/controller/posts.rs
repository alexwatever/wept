use std::io::ErrorKind;

use dioxus::Result as DxResult;
use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;

// # Modules
use super::{base::GraphQLEntity, Controller};
use crate::{
    model::{
        pagination::PageSort,
        posts::{posts_query, Post, PostQuery, Posts, PostsQuery},
    },
    State,
};

/// # Fetch a single post by ID or slug
///
/// Get a post from the WordPress GraphQL API by its ID or slug.
///
/// **Arguments**
/// * `slug` - The slug of the post to fetch.
///
/// **Returns**
/// A post.
pub(crate) async fn fetch_post(slug: String) -> DxResult<Post> {
    use crate::model::posts::post_query;

    // Create variables for the GraphQL query
    let variables = post_query::Variables {
        post_slug: slug.clone(),
    };

    // Build the GraphQL query
    let payload = PostQuery::build_query(variables);

    // Build the endpoint
    let endpoint = format!(
        "{host}/{path}",
        host = State::get_backend_host(),
        path = State::get_backend_path()
    );

    // Make the request to the GraphQL API
    let client = Client::new();
    let response = client.post(endpoint).json(&payload).send().await?;

    // Parse the response
    let response_data: Response<post_query::ResponseData> = response.json().await?;

    // Check if we have errors
    if let Some(errors) = response_data.errors {
        if !errors.is_empty() {
            let error_msg = errors
                .iter()
                .map(|e| e.message.clone())
                .collect::<Vec<String>>()
                .join(", ");

            // Use an error type that implements StdError
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("GraphQL error: {}", error_msg),
            )
            .into());
        }
    }

    // Extract the post data
    let data = response_data
        .data
        .ok_or_else(|| std::io::Error::new(ErrorKind::NotFound, "No data received"))?;

    let post_data = data
        .post
        .ok_or_else(|| std::io::Error::new(ErrorKind::NotFound, "Post not found"))?;

    // Convert the post data to a Post model
    let post = Post {
        id: post_data.id,
        content: post_data.content,
        slug: post_data.slug,
        title: post_data.title,
        date: post_data.date,
    };

    Ok(post)
}

// Implement GraphQLEntity trait for Posts
impl GraphQLEntity for Posts {
    type Variables = posts_query::Variables;
    type Query = PostsQuery;
    type ResponseData = posts_query::ResponseData;
    type Nodes = posts_query::PostsQueryPostsNodes;

    fn extract_nodes(response: Response<Self::ResponseData>) -> DxResult<Vec<Self::Nodes>> {
        let nodes = response
            .data
            .expect("No data received")
            .posts
            .expect("No posts received")
            .nodes;

        Ok(nodes)
    }

    fn from_nodes(nodes: Vec<Self::Nodes>) -> Self {
        Self::from(nodes)
    }

    fn create_variables(first: i64, after: Option<String>) -> Self::Variables {
        posts_query::Variables { first, after }
    }
}

// Implement Controller trait for Posts
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
        sort_direction: Option<PageSort>,
    ) -> DxResult<Posts> {
        Self::make_request(page_size, sort_direction).await
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
            date: None,
        }
    }
}
