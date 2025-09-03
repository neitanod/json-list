# json-list

A command-line tool to display a list of JSON objects in a compact, human-readable format.

This is a Rust port of the original [json-list](https://github.com/neitanod/json-list/blob/main/reference-implementation/json-list) bash script, with a focus on performance.

## Features

*   Reads JSON from stdin.
*   Colorized output for better readability.
*   Truncation of long values to keep the output compact.
*   Highlighting of specific columns.
*   Filtering of records using regular expressions.
*   Responsive layout that adapts to the terminal width.

## Usage

### Default

```bash
cat data.json | json-list
```

### Truncation

By default, long values are wrapped. You can use `--truncate` to truncate them instead.

```bash
cat data.json | json-list --truncate
```

You can also customize the truncation length with `--truncate-to` and `--truncate-min`.

### Highlighting

You can highlight specific columns using the `--highlight`, `--yellow`, `--green`, `--magenta`, and `--red` flags.

```bash
cat data.json | json-list --highlight=name --yellow=status
```

The primary key (defaults to `id`) is highlighted in red.

### Grep

You can filter the output using a regular expression with the `--grep` flag.

```bash
cat data.json | json-list --grep="Lobby"
```
