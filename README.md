[![Crates.io](https://img.shields.io/crates/v/restep?style=for-the-badge)](https://crates.io/crates/restep)
[![Docs.rs](https://img.shields.io/docsrs/restep?style=for-the-badge)](https://docs.rs/restep)

# restep


Restep can create highly readable APIClient.

## Usage

Automatically generates the `endpoint()` function that returns the specified endpoint.

### Basic
```rust
use restep::endpoint;

#[endpoint("/customers")]
fn simple() -> String {
    // You can use `fn endpoint() -> String` in this function.
    endpoint()
}
assert_eq!(simple(), "/customers");
```

### Path Parameters
```rust
use restep::endpoint;

struct PathParameters {
    customer_id: i32,
}

#[endpoint("/customers/{customer_id}", params = "PathParameters")]
fn dynamic_route() -> String {
    let params = PathParameters { customer_id: 1 };
    // You can use `fn endpoint(params: &PathParameters) -> String` in this function.
    endpoint(&params)
}
assert_eq!(dynamic_route(), "/customers/1");
```

## Examples

### RealWorld
```rust
use restep::endpoint;

#[derive(serde::Deserialize)]
struct Customer {
    id: i32,
    name: String,
}

struct APIClient {
    client: reqwest::Client,
}

struct PathParameters {
    customer_id: i32,
}

impl APIClient {
    #[endpoint("/customer/{customer_id}", params = "PathParameters")]
    async fn get_customer(&self, params: PathParameters) -> anyhow::Result<Customer> {
        let url = format!("{}{}", std::env::var("BASE_URL").unwrap(), endpoint(&params));
        let customer = self.client
            .get(url)
            .send()
            .await?
            .json()
            .await?;
        Ok(customer)
    }
}
```
