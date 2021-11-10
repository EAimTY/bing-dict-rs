# bing-dict

Use Bing Dictionary to translate words and phrases from Chinese to English or English to Chinese.

https://crates.io/crates/bing-dict

## Feature

- Easy to use
- Parsed paraphrase
- Regex-free

This crate uses [reqwest](https://github.com/seanmonstar/reqwest) to reach Bing Dictionary, so it can be well integrated into existing projects that use reqwest.

## Example

```rust
#[tokio::main]
async fn main() {
    let result = bing_dict::translate("dictionary").await.unwrap().unwrap();
    println!("{:?}", result);
    println!("{}", result.to_string());
}
```

## License

GNU General Public License v3.0
