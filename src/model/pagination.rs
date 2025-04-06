use parse_display::Display;

#[derive(Display, Debug)]
pub enum PageSort {
    #[display("ASC")]
    Ascending,
    #[display("DESC")]
    Descending,
}
