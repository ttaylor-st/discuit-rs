# discuit-rs

a library for easily interacting with the Discuit API.

## installation

add the following to your `Cargo.toml` file:

```toml
[dependencies.discuit-rs]
git = "https://github.com/ttaylor-st/discuit-rs.git"
```

discuit-rs will eventually be available on crates.io, but for now you can use
the git repository.

<!---
## basic usage

```rust
use std::env;
use discuit_rs::DiscuitClient;

#[tokio::main]
async fn main() {
    let base_url = env::var("DISCUIT_BASE_URL").unwrap();
    let username = env::var("DISCUIT_USERNAME").unwrap();
    let password = env::var("DISCUIT_PASSWORD").unwrap();

    let mut client = DiscuitClient::new(&base_url);
    client.initialize().await.unwrap();
    client.login(&username, &password).await.unwrap();
    let user = client.get_user().await.unwrap();
    println!("{:?}", user);
}
```
-->

<!--- TODO: you can view the documentation [here](https://ttaylor-st.github.io/discuit-rss))and more examples in the [`examples`](examples) directory (soon). -->



## license

discuit-rs is licensed under MIT, view [LICENSE](LICENSE) for more information.
