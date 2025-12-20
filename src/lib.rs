/*
  Copyright (c) 2025 iampi31415

  This file is part of mdbook-fetch

  mdbook-fetch is free software: you can redistribute it and/or modify
  it under the terms of the GNU Lesser General Public License version 2.1
  as published by the Free Software Foundation and appearing in the file
  LICENSE.LGPL included in the packaging of this file.

  This program is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  GNU Lesser General Public License for more details.
*/
use std::sync::LazyLock;

use mdbook_preprocessor::{
    Preprocessor, PreprocessorContext,
    book::{Book, Chapter},
    errors::Result,
};
use pulldown_cmark::{Event, Parser};
use regex::{Captures, Regex};
use reqwest::blocking::get as get_reqwest;

/// Build the regex only once.
static RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\{\{\s*#fetch\s+([^\s}{]{5,200})\s*\}\}").unwrap()
});
/// Preprocessor that fetches remote markdown files
pub struct Fetch;

impl Preprocessor for Fetch {
    fn name(&self) -> &str {
        "fetch"
    }

    /// Modify chapters replacing `{{#fetch URLs}}` by the .md content.
    fn run(
        &self,
        ctx: &PreprocessorContext,
        mut book: Book,
    ) -> Result<Book> {
        // book.toml option for this preprocessor.
        let option = "preprocessor.fetch.disable";
        match ctx.config.get::<bool>(option) {
            // `Ok(None)` is field unset.
            Ok(None) | Ok(Some(false)) => {
                book.for_each_chapter_mut(|ch| {
                    match include_markdown(ch) {
                        Ok(s) => ch.content = s,
                        Err(e) => {
                            eprintln!("failed to process chapter: {e:?}")
                        }
                    }
                });
                Ok(book)
            }
            Ok(Some(true)) => Ok(book),
            Err(err) => Err(err.into()),
        }
    }
    /// Preprocess Markdown, regardless of
    /// the final output being .html or .md
    fn supports_renderer(&self, renderer: &str) -> Result<bool> {
        Ok(renderer == "html" || renderer == "md")
    }
}

/// Modify the standard input when it matches URLs.
fn include_markdown(ch: &mut Chapter) -> Result<String> {
    let mut buf = String::with_capacity(ch.content.len());

    // Iterator over events
    let parser = Parser::new(&ch.content).map(|e| match e {
        Event::Text(text) => {
            let fetched_markdown = find_and_replace_fetches(&text);
            Event::Text(fetched_markdown.into())
        }
        _ => e,
    });
    Ok(pulldown_cmark_to_cmark::cmark(parser, &mut buf).map(|_| buf)?)
}

fn fetch(url: String) -> String {
    get_reqwest(url).unwrap().text().unwrap()
}
/// Search through `text`, replace any `{{#fetch <URL>}}` with the
/// content the URL points to.
/// `<URL>` must be web address to _raw_ markdown.
fn find_and_replace_fetches(text: &str) -> String {
    RE.replace(text, |caps: &Captures| {
        let mut r: String = fetch(caps[1].into());
        r.insert_str(0, "\n");
        r
    })
    .to_string()
}

#[cfg(test)]
mod test {

    use regex::Match;

    use super::*;

    #[test]
    fn test_regex() {
        let input_str: &str = r#"some text and even more but now 
            // Should fail: blank in `// a.`
            {{ #fetch https:// abc.def.g/mypath/to.md }} 
            // Should pass {{ #fetch https://abc.def.g/mypath/to.md }} 
            // Should pass\n
{{#fetch https://abc.def.ga.b.c/mypath/to.md}}
            // Should pass: `http` is accepted
            {{ #fetch http://this.is.insecure/fails/to.md }}
            // Should pass: {{#fetch https://github.com/rvben/rumdl/blob/main/docs/markdownlint-comparison.md}}
            
            The source of the following content is at <https://github.com/rust-lang/mdBook/> {{#fetch https://raw.githubusercontent.com/rust-lang/mdBook/7b29f8a7174fa4b7b31536b84ee62e50a786658b/README.md }}

        //"#;
        fn find_markdown_urls(str_file: &str) -> Vec<&str> {
            // I did not find out a way to use the same regex
            // since `regex!` and `regex_replace_all!` need a
            // literal. And using `static reg=..` was too hard.
            RE.find_iter(str_file).map(|m: Match| m.as_str()).collect()
        }

        let result = find_markdown_urls(input_str);
        let last_result = *result.last().unwrap();
        assert_eq!(
            last_result,
            "{{#fetch https://raw.githubusercontent.com/rust-lang/mdBook/7b29f8a7174fa4b7b31536b84ee62e50a786658b/README.md }}"
        );
        assert_eq!(result.len(), 5)
    }
    #[test]
    fn test_full_run() {
        let input_json = r##"[
                {
                    "root": "/path/to/book",
                    "config": {
                        "book": {
                            "authors": ["AUTHOR"],
                            "language": "en",
                            "src": "src",
                            "title": "TITLE"
                        },
                        "preprocessor": {
                            "nop": {}
                        }
                    },
                    "renderer": "html",
                    "mdbook_version": "0.4.21"
                },
                {
                    "items": [
                        {
                            "Chapter": {
                                "name": "Chapter 1",
                                "content": "The source of the following content is at <https://github.com/rust-lang/mdBook/> {{#fetch https://raw.githubusercontent.com/rust-lang/mdBook/7b29f8a7174fa4b7b31536b84ee62e50a786658b/README.md }}",
                                "number": [1],
                                "sub_items": [],
                                "path": "chapter_1.md",
                                "source_path": "chapter_1.md",
                                "parent_names": []
                            }
                        }
                    ]
                }
            ]"##;
        let input_json = input_json.as_bytes();

        let (ctx, src_book) =
            mdbook_preprocessor::parse_input(input_json).unwrap();
        let result = Fetch.run(&ctx, src_book);
        assert!(result.is_ok());

        let dst_book = result.unwrap();
        let first = &dst_book.chapters().next().unwrap().content;
        assert!(first.contains("The source of the following"));
        assert!(first.contains("mdBook is a utility to create modern online books from Markdown files."))
    }
}
