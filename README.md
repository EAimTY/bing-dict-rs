# bing-dict

Use Bing Dictionary to translate words and phrases between English and Chinese

[![Version](https://img.shields.io/crates/v/bing-dict.svg?style=flat)](https://crates.io/crates/bing-dict)
[![Documentation](https://img.shields.io/badge/docs-release-brightgreen.svg?style=flat)](https://docs.rs/bing-dict)
[![License](https://img.shields.io/crates/l/bing-dict.svg?style=flat)](https://github.com/EAimTY/bing-dict-rs/blob/master/LICENSE)

## Example

```rust
#[tokio::main]
async fn main() {
    let result = bing_dict::translate("dictionary").await.unwrap().unwrap();
    println!("{result}");
}
```

## License

GNU General Public License v3.0
