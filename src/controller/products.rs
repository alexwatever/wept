use dioxus::Result as DxResult;
use reqwest::get;

// # Modules
use crate::model::{pagination::PageSort, product::Product};

pub(crate) async fn fetch_products(count: usize, sort: PageSort) -> DxResult<Vec<Product>> {
    let url: String = format!("https://fakestoreapi.com/products/?sort={sort}&limit={count}");
    let request: Vec<Product> = get(&url).await?.json().await?;

    Ok(request)
}
