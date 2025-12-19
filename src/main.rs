/*
  Copyright (c) 2025 iampi31415

  This file is part of mdbook-fetch.

  mdbook-fetch is free software: you can redistribute it and/or modify
  it under the terms of the GNU Lesser General Public License version 2.1
  as published by the Free Software Foundation and appearing in the file
  LICENSE.LGPL included in the packaging of this file.

  This program is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  GNU Lesser General Public License for more details.
*/

//! `mdbook-fetch` is `mdbook` plugin to include remote markdown.
//! The syntax is `{{#remote <URL>}}`.
//! The remote content must be raw markdown, not HTML.
use std::{io, process};

use clap::{Arg, ArgMatches, Command};
use mdbook_fetch::Fetch;
use mdbook_preprocessor::{Preprocessor, errors::Result};
use semver::{Version, VersionReq};

fn main() {
    // `.get_matches()` extracts Args.
    let c_line = make_app().get_matches();

    let pre = Fetch;

    if let Some(sub_args) = c_line.subcommand_matches("supports") {
        // 1st run
        handle_supports(&pre, sub_args);
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
fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> ! {
    let renderer = sub_args.get_one::<String>("renderer").expect(
        "`<renderer>` is a required argument and should be specified.",
    );
    let supported = pre.supports_renderer(renderer).unwrap();
    // Signals to `mdbook` whether we support this `<renderer>`
    // `0` for "yes", `1` for "no".
    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}
/// Think of a command like `echo "hello world"`
/// And all it's possible behaviours.
/// Now we describe that.
fn make_app() -> Command {
    // Define our CLI application.
    let command = Command::new("mdbook-fetch") // name
        .about(
            "`mdbook-fetch` is an `mdbook` plugin to \
            include remote markdown.\nThe syntax is \
            `{{#remote <URL>}}`.\nThe remote content must be \
            raw markdown, not HTML.",
        );

    command.subcommand(
        // create a `supports` sub-command
        // As `test` in `cargo test`.
        Command::new("supports")
            // positional argument after supports
            // `mdbook-fetch supports <renderer>`
            .arg(Arg::new("renderer").required(true))
            // Help for the `supports` subcommand
            .about(
                "Check whether a renderer is supported by \
                    this preprocessor",
            ),
    )
}
