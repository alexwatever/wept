use dioxus::Result as DxResult;
use graphql_client::Response;

// # Modules
use super::{base::GraphQLEntity, Controller};
use crate::model::{
    pagination::PageSort,
    posts::{posts_query, Post, Posts, PostsQuery},
};

/// # Fetch a single post by ID
///
/// Get a post from the WordPress GraphQL API by its ID.
///
/// **Arguments**
/// * `post_id` - The ID of the post to fetch.
///
/// **Returns**
/// A post.
pub(crate) async fn fetch_post(post_id: String) -> DxResult<Post> {
    // This is a placeholder implementation
    // In a real implementation, you would use a GraphQL query to fetch the post by ID

    let post = Post {
        id: post_id,
        content: Some("This is a sample post content fetched from the API.".to_string()),
        slug: Some("sample-post".to_string()),
        title: Some("Sample Post".to_string()),
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
        }
    }
}
