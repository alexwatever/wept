use dioxus::Result as DxResult;
use reqwest::get;

// # Modules
use crate::model::{
    pagination::PageSort,
    product::{Product, Products},
};

pub(crate) async fn _fetch_products(count: usize, sort: PageSort) -> DxResult<Vec<Product>> {
    let url: String = format!("https://fakestoreapi.com/products/?sort={sort}&limit={count}");

    let request: Vec<Product> = get(&url).await?.json().await?;

    Ok(request)
}

impl Products {
    /// # Fetch Products
    ///
    /// Get a list of products from the WordPress GraphQL API.  
    ///
    /// **Arguments**  
    ///
    /// * `page_count` - The number of products to fetch.
    /// * `page_sort` - The sort order of the products.
    ///
    /// **Returns**  
    ///
    /// A list of products.
    pub(crate) async fn get(page_count: usize, page_sort: PageSort) -> DxResult<Products> {
        let url: String =
            format!("https://fakestoreapi.com/products/?sort={page_sort}&limit={page_count}");

        let request: Vec<Product> = get(&url).await?.json().await?;
        let request: Products = Products(request);

        // TODO; Get WordPress products

        Ok(request)
    }
}
