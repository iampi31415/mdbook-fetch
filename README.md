# mdbook-fetch
<img height=35 src="./gplv3.svg" alt="GPLv3 Logo: a red background with the text 'GPLv3' in white font, and an additional 'or later' in black font."/>
This is a work in progress, so it's not published on <https://crates.io>.

The way to use it as fragile as it is now, is:

```bash
cargo install --git https://github.com/iampi31415/mdbook-fetch --locked
```

Then add it to `[mdbook.fetch]` table in the `book.toml`, and `mdbook` will use it to fetch remote markdown. 

The way to specify the markdown is `{{#fetch <URL>}}` where `<URL>` is to a raw markdown file, for example `{{#fetch https://raw.githubusercontent.com/rust-lang/mdBook/7b29f8a7174fa4b7b31536b84ee62e50a786658b/README.md}}`.

The requests are made one by one as they are parsed, so don't overuse it.

To disable it if it becomes annoying in development, add `disable=true` to the `[mdbook.fetch]` table.

## Recommendation

Add a line to the source of the content, and check the license of their content allows reusing it.

## Limitations

It does not pre-process the markdown received from the URL so it's your responsibility to ensure it's safe to use.

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
