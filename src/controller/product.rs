use dioxus::Result as DxResult;
use reqwest::get;

// # Modules
use crate::model::product::Product;

pub(crate) async fn fetch_product(product_id: usize) -> DxResult<Product> {
    let url: String = format!("https://fakestoreapi.com/products/{product_id}");
    let request: Product = get(&url).await?.json().await?;

    Ok(request)
}
