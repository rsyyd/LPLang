# Functions

```lp
fn add(a: i64, b: i64) -> i64 {
    a + b
}

# Async functions
async fn fetch(url: String) -> Result<Response, Error> {
    http::get(url).await
}
```