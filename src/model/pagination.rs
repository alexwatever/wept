use parse_display::Display;

// # Pagination Sort
#[allow(unused)]
#[derive(Display, PartialEq, Eq, Clone, Debug)]
pub(crate) enum PageSort {
    #[display("desc")]
    Descending,
    #[display("asc")]
    Ascending,
}
