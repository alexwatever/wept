use serde::{Deserialize, Serialize};

// Modules
use crate::{
    graphql::models::post::{
        post_query::PostQueryPost,
        posts_query::{PostsQueryPosts, PostsQueryPostsNodes, PostsQueryPostsPageInfo},
    },
    models::pagination::Pagination,
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

/// Collection of posts with pagination information
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct Posts {
    pub posts: Vec<Post>,
    pub page_info: Option<Pagination>,
}

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
        let page_info: Option<Pagination> = Some(Pagination::from(posts.page_info));
        let posts: Vec<Post> = posts.nodes.into_iter().map(Post::from).collect();

        Self { posts, page_info }
    }
}

impl From<PostsQueryPostsPageInfo> for Pagination {
    /// Convert a PostsQueryPostsPageInfo to a Pagination
    ///
    /// **Arguments**
    ///
    /// * `page_info` - The GraphQL page info to convert
    ///
    /// **Returns**
    ///
    /// * `Pagination` - The converted Pagination
    fn from(page_info: PostsQueryPostsPageInfo) -> Self {
        Self {
            end_cursor: page_info.end_cursor,
            has_next_page: page_info.has_next_page,
        }
    }
}
