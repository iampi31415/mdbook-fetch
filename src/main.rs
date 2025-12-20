/*
Copyright (c) 2025 iampi31415

This file is part of mdbook-fetch

mdbook-fetch is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

mdbook-fetch is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

//! `mdbook-fetch` is `mdbook` plugin to fetch remote markdown.
//! The syntax is `{{#remote <URL>}}`.
//! The remote content must be raw markdown, not HTML.
use std::{io, process};

use mdbook_fetch::Fetch;
use mdbook_preprocessor::{Preprocessor, errors::Result};
use semver::{Version, VersionReq};

fn main() {
    let mut c_line = std::env::args(); // command "line"
    let first_arg = c_line.nth(1); // 0 is the command name.

    let pre = Fetch;

    if let Some("supports") = first_arg.as_deref() {
        // 1st run
        let renderer =
            c_line.next().expect("Renderer should be defined.");
        handle_supports(&pre, &renderer);
    } else if let Some(val) = first_arg.as_deref() {
        // 2nd run. Mutate or error.
        eprintln!(r#"Expected "supports" but found {val:?}"#);
        process::exit(1);
    } else if let Err(e) = handle_preprocessing(&pre) {
        // 2nd run. Mutate or error.
        eprintln!("{e:?}");
        process::exit(1);
    }
}

/// Ensures user's `mdbook` and this plugin's
/// `mdbook-preprocessor` are "semver compatible".
/// `pre` is the type implementing `Preprocessor` trait.
fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<()> {
    // Here it waits for the mdbook input.
    let (ctx, book) = mdbook_preprocessor::parse_input(io::stdin())?;

    let user_version = Version::parse(&ctx.mdbook_version)?;
    let plugin_version =
        VersionReq::parse(mdbook_preprocessor::MDBOOK_VERSION)?;

    if !plugin_version.matches(&user_version) {
        eprintln!(
            "Warning: The {} plugin uses version {} of `mdbook`, \
             but you are using version {}.",
            pre.name(),
            mdbook_preprocessor::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

/// Inform `mdbook` whether we support the renderer passed.
/// `0` if it supports <renderer>
/// `1` if it does not support <renderer>
fn handle_supports(pre: &dyn Preprocessor, renderer: &str) -> ! {
    // In our case this never fails, but it's the signature.
    let supported = pre.supports_renderer(renderer).unwrap();
    // Signals to `mdbook` whether we support this `<renderer>`
    // `0` for "yes", `1` for "no".
    if supported {
        process::exit(0);
    } else {
        eprintln!("The renderer {renderer} is not supported. Exiting.");
        process::exit(1);
    }
}
