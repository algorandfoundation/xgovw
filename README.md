xgovw
====

An engine which ensures [xGov](https://github.com/algorandfoundation/xGov) meet certain requirements.

## Getting Started

To install `xgovw` and validate the xGov repository:

```console
git clone git@github.com:algorandfoundation/xgovw.git
cargo install --path=xgovw xgovw
xgovw /path/to/xGov
```

```
USAGE:
    xgovw [OPTIONS] [SOURCES]...

ARGS:
    <SOURCES>...    Files and/or directories to check

OPTIONS:
        --format <FORMAT>     Output format [default: text] [possible values: text, json]
    -h, --help                Print help information
        --lints <LINTS>       Additional lints to enable
        --list-lints          List all available lints
        --no-default-lints    Do not enable the default lints
```



## Demo

### Example xgov

```markdown
---
id: 1
author: Stéphane Barroso(@sudoweezy)
email: stephane@algorand.co
address: AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAY5HFKQ
status: Final
---

## Introduction
I can contribute to the ecosystem.

## Social profiles
@SudoWeezy Github

## Relevant Experience
Worked for Algorand Foundation.

## Projects Affiliation
Algorand Foundation

## Additional Information
Here are some details.

```

### Output

```
error[markdown-order-section]: section `Team` must come after `Abstract`
  --> /tmp/xgov_council-1.md
   |
12 | ## Team
```

## Lints

| id                                  | Description                                                                                   |
|-------------------------------------|-----------------------------------------------------------------------------------------------|
| `preamble-file-name`                | The file name reflects the xgov number.                                                       |
| `preamble-req`                      | All required preamble headers are present.                                                    |
| `preamble-order`                    | The preamble headers are in the correct order.                                                |
| `preamble-no-dup`                   | There are no duplicate headers.                                                               |
| `preamble-trim`                     | There is no extra whitespace around preamble fields.                                          |
| `preamble-id`                         | The `id` header is a positive integer                                                     |
| `preamble-len-title`                | The `title` header isn't too long.                                                            |
| `preamble-author`                   | The author header is correctly formatted, and there is at least one GitHub user listed.       |
| `preamble-list-author`              | The `author` header is a correctly formatted comma-separated list.                            |
| `preamble-len-company_name`         | The `company_name` header isn't too long.                                                     |
| `preamble-len-amount_requested`         | The `amount_requested` header is a at least 5 digits. |
| `preamble-amount_requested`         | The `amount_requested` header is a positive integer                                           |
| `preamble-enum-category`            | The `category` header is a recognized value.                                                  |
| `preamble-enum-focus_area`          | The `focus_area` header is a recognized value.                                                |
| `preamble-enum-open_source`         | The `open_source` header is a recognized value.                                               |
| `preamble-enum-status`              | The `status` header is a recognized value.                                                    |
| `markdown-req-section`              | Required sections are present in the body of the proposal.                                    |
| `markdown-order-section`            | There are no extra sections and the sections are in the correct order.                        |
| `markdown-re-xgov-not-xgov`         | Other xgovs are referenced using xGov-X, not xgov-X.                                          |
| `markdown-re-xgov-dash`             | Other xgovs are referenced using xGov-X, not XGOVX or xGov X.                                  |
| `markdown-link-first`               | First mention of an xgov must be a link.                                                      |
| `markdown-rel-links`                | All URLs in the page are relative. (or use the html <a href="uri">Topic<a>) format            |


## JavaScript / WebAssembly

`xgovw-lint-js` packages `xgovw` as an npm package, for use in JavaScript / TypeScript.

You can find the [package on npm](https://www.npmjs.com/package/xgovw-lint-js).

### Building & Publishing

`cargo install --path=xgovw xgovw`

```bash
cd xgovw-lint-js
wasm-pack build -t nodejs
wasm-pack publish -t nodejs
```
