use dioxus::prelude::*;
use graphql_client::GraphQLQuery;
use parse_display::FromStr;
use serde::{Deserialize, Serialize};
use std::fmt::{Display as FmtDisplay, Formatter, Result as FmtResult};

use crate::view::components::entity_list::EntityDisplay;

/// # Post
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub(crate) struct Post {
    pub(crate) id: String,
    pub(crate) content: Option<String>,
    pub(crate) slug: Option<String>,
    pub(crate) title: Option<String>,
}

/// # Posts
#[derive(Debug, PartialEq)]
pub(crate) struct Posts(pub Vec<Post>);

/// # Posts GraphQL Query
#[derive(GraphQLQuery, Debug)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/posts_query.graphql",
    response_derives = "Debug"
)]
pub struct PostsQuery;

/// # Post Rating
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub(crate) struct PostRating {
    pub(crate) rate: f32,
    pub(crate) count: u32,
}

impl FmtDisplay for PostRating {
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

/// # Post Size
#[derive(Default, FromStr, Debug)]
#[display(style = "snake_case")]
pub(crate) enum PostSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl EntityDisplay for Post {
    fn render(&self) -> Element {
        let Post {
            id,
            content,
            slug,
            title,
            ..
        } = self.clone();

        rsx! {
            section { class: "p-2 m-2 shadow-lg ring-1 rounded-lg flex flex-row place-items-center hover:ring-4 hover:shadow-2xl transition-all duration-200",
                div { class: "pl-4 text-left text-ellipsis",
                    a {
                        href: "/posts/{id}",
                        class: "w-full text-center font-bold text-xl",
                        "{title:?}"
                    }
                    p {
                        class: "w-full text-sm text-gray-500",
                        "{slug:?}"
                    }
                    p {
                        class: "w-full text-sm overflow-hidden line-clamp-3",
                        "{content:?}"
                    }
                }
            }
        }
    }
}
