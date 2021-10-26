//! # bing-dict
//!
//! Use Bing Dict to translate from Chinese to English or from English to Chinese.

use std::str;
use subslice::SubsliceExt;
use thiserror::Error;

/// Translate a word through Bing Dict. Return `Ok(None)` if the word is not found in Bing Dict.
///
/// # Examples
///
/// ```
/// #[tokio::main]
/// async fn main() {
///     let result = bing_dict::translate("dictionary").await.unwrap();
///     assert_eq!(
///         Some(String::from("美[ˈdɪkʃəˌneri]，英[ˈdɪkʃən(ə)ri]，n. 词典；字典；辞书；专业术语大全； 网络释义： 辞典；字典中管理；字典对象；")),
///         result,
///     );
/// }
/// ```
pub async fn translate<S: AsRef<str>>(input: S) -> Result<Option<String>, Error> {
    let url = format!(
        "https://www.bing.com/dict/search?mkt=zh-cn&q={}",
        input.as_ref()
    );

    let response = reqwest::get(url).await?.bytes().await?;

    if let Some(start) = response.find(br#"<meta name="description" content=""#) {
        if let Some(end) = response[start..].find(br#"" />"#) {
            if response.len() > start + 40 {
                if &response[start + 34..start + 40] != b"\xE8\xAF\x8D\xE5\x85\xB8"
                    && &response[start + 34..start + 61] != b"\xE5\xBF\x85\xE5\xBA\x94\xE8\xAF\x8D\xE5\x85\xB8\xEF\xBC\x8C\xE4\xB8\xBA\xE6\x82\xA8\xE6\x8F\x90\xE4\xBE\x9B"
                {
                    let input_length = html_escape::encode_text(&input).len();

                    return Ok(Some(
                        str::from_utf8(&response[start + input_length + 70..start + end])?
                            .trim()
                            .to_string(),
                    ));
                } else {
                    return Ok(None);
                }
            }
        }
    }

    Err(Error::PageError)
}

/// The Errors that may occur
#[derive(Error, Debug)]
pub enum Error {
    #[error(r#"no <meta name="description" /> found in page"#)]
    PageError,
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    Utf8Error(#[from] str::Utf8Error),
}

#[cfg(test)]
mod tests {
    use crate::translate;

    #[tokio::test]
    async fn eng_to_chi() {
        assert_eq!(
            Some(String::from("美[ˈdɪkʃəˌneri]，英[ˈdɪkʃən(ə)ri]，n. 词典；字典；辞书；专业术语大全； 网络释义： 辞典；字典中管理；字典对象；")),
            translate("dictionary").await.unwrap(),
        );
    }

    #[tokio::test]
    async fn chi_to_eng() {
        assert_eq!(
            Some(String::from("拼音[cí diǎn]，na. dictionary; lexicon； 网络释义： Thesaurus; Dictionaries; Word dictionary；")),
            translate("词典").await.unwrap(),
        );
    }

    #[tokio::test]
    async fn no_paraphrase() {
        assert_eq!(None, translate("yranoitcid").await.unwrap());
    }
}
