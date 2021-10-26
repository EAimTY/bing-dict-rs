# bing-dict

Use Bing Dict to translate from Chinese to English or from English to Chinese.

https://crates.io/crates/bing-dict

## Example

```rust
#[tokio::main]
async fn main() {
    let result = bing_dict::translate("dictionary").await.unwrap();
    assert_eq!(
        Some(String::from("美[ˈdɪkʃəˌneri]，英[ˈdɪkʃən(ə)ri]，n. 词典；字典；辞书；专业术语大全； 网络释义： 辞典；字典中管理；字典对象；")),
        result,
    );
}
```

## License

GNU General Public License v3.0
