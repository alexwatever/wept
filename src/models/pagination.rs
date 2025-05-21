use serde::{Deserialize, Serialize};

/// Represents pagination information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Pagination {
    /// End cursor
    pub end_cursor: Option<String>,
    /// Has next page
    pub has_next_page: bool,
}
