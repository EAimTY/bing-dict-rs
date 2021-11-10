//! # bing-dict
//!
//! Use Bing Dictionary to translate words and phrases from Chinese to English or English to Chinese.
//!
//! # Examples
//!
//! ```
//! #[tokio::main]
//! async fn main() {
//!     let result = bing_dict::translate("dictionary").await.unwrap().unwrap();
//!     println!("{:?}", result);
//!     println!("{}", result.to_string());
//! }
//! ```

use std::{fmt, str};
use subslice::SubsliceExt;
use thiserror::Error;

/// Translate a word / phrase using Bing Dictionary. Return `Ok(None)` if the word can not be found.
///
/// # Examples
///
/// ```
/// #[tokio::main]
/// async fn main() {
///     let result = bing_dict::translate("dictionary").await.unwrap().unwrap();
///     println!("{:?}", result);
/// }
/// ```
pub async fn translate<S: AsRef<str>>(input: S) -> Result<Option<Paraphrase>, Error> {
    let url = format!(
        "https://www.bing.com/dict/search?mkt=zh-cn&q={}",
        input.as_ref()
    );

    let response = reqwest::get(url).await?.bytes().await?;

    if let Some(start) = response.find(br#"<meta name="description" content=""#) {
        let start = start + 34;
        if let Some(end) = response[start..].find(br#"" />"#) {
            let description = &response[start..start + end];
            let preamble_len = html_escape::encode_text(&input).len() + 36;

            if description.starts_with(b"\xE5\xBF\x85\xE5\xBA\x94\xE8\xAF\x8D\xE5\x85\xB8\xE4\xB8\xBA\xE6\x82\xA8\xE6\x8F\x90\xE4\xBE\x9B")
                && description.len() > preamble_len
            {
                let paraphrase = str::from_utf8(&description[preamble_len..])?
                    .trim();

                return Ok(Some(Paraphrase::parse(input.as_ref(), paraphrase)));
            } else {
                return Ok(None);
            }
        }
    }

    Err(Error::PageError)
}

/// The paraphrase of a word / phrase.
/// You can read its fields manually or just call `.to_string()` to get the paraphrase as a string.
#[derive(Debug, Clone)]
pub struct Paraphrase {
    pub query: String,
    pub pronunciations: Vec<String>,
    pub genders: Vec<String>,
}

impl Paraphrase {
    pub(crate) fn parse(query: &str, paraphrase: &str) -> Self {
        let mut pronunciations = Vec::new();
        let mut genders = Vec::new();

        for part in paraphrase.split('，') {
            if part.starts_with('英') || part.starts_with('美') || part.starts_with("拼音") {
                pronunciations.push(part.to_string());
            } else {
                for gender in part.split("； ") {
                    genders.push(gender.trim_end_matches('；').to_string())
                }
            }
        }

        Self {
            query: query.to_owned(),
            pronunciations,
            genders,
        }
    }

    /// Get pronunciations as a `String`
    pub fn pronunciations_to_string(&self) -> String {
        self.pronunciations.join("，")
    }

    /// Get genders as a `String`
    pub fn genders_to_string(&self) -> String {
        self.genders.join("\n")
    }
}

impl fmt::Display for Paraphrase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pronunciations = if !self.pronunciations.is_empty() {
            let mut pronunciations = self.pronunciations_to_string();
            pronunciations.push('\n');
            pronunciations
        } else {
            String::new()
        };

        write!(
            f,
            "{}\n{}{}",
            self.query,
            pronunciations,
            self.genders_to_string()
        )
    }
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
    async fn chi_to_eng() {
        assert!(translate("词典").await.unwrap().is_some());
    }

    #[tokio::test]
    async fn chi_to_eng_no_result() {
        assert!(translate("没有在必应词典中找到结果")
            .await
            .unwrap()
            .is_none());
    }

    #[tokio::test]
    async fn eng_to_chi() {
        assert!(translate("dictionary").await.unwrap().is_some());
    }

    #[tokio::test]
    async fn eng_to_chi_no_result() {
        assert!(translate("yranoitcid").await.unwrap().is_none());
    }
}
