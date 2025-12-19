# mdbook-fetch

<img height=35 src="./gplv3.svg" alt="GPLv3 Logo: a red background with the text 'GPLv3' in white font, and an additional 'or later' in black font."/>

`mdbook-fetch` is a preprocessor for [`mdbook`](https://github.com/rust-lang/mdBook).

It is a work in progress, so it's not published on [https://crates.io](https://crates.io).

**What does it  do**: It fetches remote markdown files and adds them to your book. The idea it that we can reuse content.

## Install

```bash
cargo install --git https://github.com/iampi31415/mdbook-fetch --locked
```

## Usage

1. Add to your `book.toml` the `[preprocessor.fetch]` table and now `mdbook` will use it to fetch remote markdown. 
2. In any of your markdown files (files under `src/`) use `{{#fetch <URL>}}` where `<URL>` is to a raw markdown file. 
    - For example 
    ```md
    This chapter is copied from [this source](https://github.com/rust-lang/mdBook)

    {{#fetch https://raw.githubusercontent.com/rust-lang/mdBook/7b29f8a7174fa4b7b31536b84ee62e50a786658b/README.md}}`.
    ```

The requests for remote files are serial (one after another), so don't overuse it.

> [!TIP]
> To disable it if it becomes annoying in development, add `disable=true` to the `[mdbook.fetch]` table.

## Recommendation

Always add a line to the source of the remote content, and check that the license of their content allows reusing it.

## Limitations

It does not preprocess the markdown received from the URL so it's your responsibility to ensure it's safe to use.

Importantly, it won't work for markdown files with images (unless the images have full URLs.)

This could potentially be fixed by guessing where are they stored, but it's a bit of effort.

## License
GNU General Public License v3.0 or later, see [COPYING](./COPYING).

## Contributing

Any contributions are under the same license "GNU General Public License v3.0 or later".

This is a _copyleft_ license, so modified or verbatim copies of this work must retain the license. On its own or embedded in software.

This is a way to ensure our software remains _free_, and a reminder to others restricting us that we are still fighting.

If put in a plugin and use the output of it, no specific license applies to the output, since the output does not contain the program.

-------

No AI code please.
