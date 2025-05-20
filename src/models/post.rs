use serde::{Deserialize, Serialize};

// Modules
use crate::graphql::queries::post::{
    post_query::PostQueryPost,
    posts_query::{PostsQueryPosts, PostsQueryPostsNodes},
};

/// Post entity representing a WordPress post
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Post {
    /// Post ID
    pub id: String,
    /// Post content
    pub content: Option<String>,
    /// Post slug
    pub slug: Option<String>,
    /// Post title
    pub title: Option<String>,
    /// Post date
    pub date: Option<String>,
}

impl From<PostQueryPost> for Post {
    /// Convert a PostQueryPost to a Post
    ///
    /// **Arguments**
    ///
    /// * `post` - The GraphQL response to convert
    ///
    /// **Returns**
    ///
    /// * `Post` - The converted Post
    fn from(post: PostQueryPost) -> Self {
        Self {
            id: post.id,
            content: post.content,
            slug: post.slug,
            title: post.title,
            date: post.date,
        }
    }
}

impl From<PostsQueryPostsNodes> for Post {
    /// Convert a PostsQueryPostsNodes to a Post
    ///
    /// **Arguments**
    ///
    /// * `post` - The GraphQL post to convert
    ///
    /// **Returns**
    ///
    /// * `Post` - The converted Post
    fn from(post: PostsQueryPostsNodes) -> Self {
        Self {
            id: post.id,
            content: post.content,
            slug: post.slug,
            title: post.title,
            date: post.date,
        }
    }
}

/// Collection of posts
#[derive(Debug, PartialEq)]
pub struct Posts(pub Vec<Post>);

impl From<PostsQueryPosts> for Posts {
    /// Convert a PostsQueryPosts to Posts
    ///
    /// **Arguments**
    ///
    /// * `posts` - The GraphQL posts to convert
    ///
    /// **Returns**
    ///
    /// * `Posts` - The converted Posts
    fn from(posts: PostsQueryPosts) -> Self {
        Posts(posts.nodes.into_iter().map(Post::from).collect())
    }
}
