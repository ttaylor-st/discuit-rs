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


## basic usage

```rust
use discuit_rs::client::*;

#[tokio::main]
async fn main() {
    let base_url = "https://discuit.net";
    let mut client = DiscuitClient::new(base_url);
    let response = client.initialize().await;
    println!("{:?}", response);
}
```

<!--- TODO: you can view the documentation [here](https://ttaylor-st.github.io/discuit-rss))and more examples in the [`examples`](examples) directory (soon). -->



## license

discuit-rs is licensed under MIT, view [LICENSE](LICENSE) for more information.
