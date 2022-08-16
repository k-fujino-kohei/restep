//!
//! Restep can create highly readable APIClient.
//!
//! # Usage
//!
//! Automatically generates the `endpoint()` function that returns the specified endpoint.
//!
//! ## Basic
//! ```
//! use restep::endpoint;
//!
//! #[endpoint("/customers")]
//! fn simple() -> String {
//!     // You can use `fn endpoint() -> String` in this function.
//!     endpoint()
//! }
//! assert_eq!(simple(), "/customers");
//! ```
//!
//! ## Path Parameters
//! ```
//! use restep::endpoint;
//!
//! struct PathParameters {
//!     customer_id: i32,
//! }
//!
//! #[endpoint("/customers/{customer_id}", params = "PathParameters")]
//! fn dynamic_route() -> String {
//!     let params = PathParameters { customer_id: 1 };
//!     // You can use `fn endpoint(params: &PathParameters) -> String` in this function.
//!     endpoint(&params)
//! }
//! assert_eq!(dynamic_route(), "/customers/1");
//! ```
//!
//! # Examples
//!
//! ## RealWorld
//! ```
//! use restep::endpoint;
//!
//! #[derive(serde::Deserialize)]
//! struct Customer {
//!     id: i32,
//!     name: String,
//! }
//!
//! struct APIClient {
//!     client: reqwest::Client,
//! }
//!
//! struct PathParameters {
//!     customer_id: i32,
//! }
//!
//! impl APIClient {
//!     #[endpoint("/customer/{customer_id}", params = "PathParameters")]
//!     async fn get_customer(&self, params: PathParameters) -> anyhow::Result<Customer> {
//!         let url = format!("{}{}", std::env::var("BASE_URL").unwrap(), endpoint(&params));
//!         let customer = self.client
//!             .get(url)
//!             .send()
//!             .await?
//!             .json()
//!             .await?;
//!         Ok(customer)
//!     }
//! }
//! ```

mod endpoint;

use crate::endpoint::parse_attr;
use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, ItemFn};

///
/// Creates a function that returns the specified path.
///
/// # Syntax
/// ```
/// #[endpoint("path"[, attributes])]
/// ```
///
/// # Attributes
/// - `path`: endpoint. If an embedded variable is enclosed in braces, the variable must be a field of `params`.
/// - `name = "function name"`: Name for auto-generated function. Default is `endpoint`
/// - `params = "argument type"`: Argument type for auto-generated function.
///
#[proc_macro_attribute]
pub fn endpoint(attr: TokenStream, item: TokenStream) -> TokenStream {
    parse_attr(
        parse_macro_input!(attr as AttributeArgs),
        parse_macro_input!(item as ItemFn),
    )
    .into()
}
