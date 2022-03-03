# hs

**hs** ("history, sorted") is a tool that parses a timestamped Bash history file and displays it in chronological order as tab-separated values.

## Why not use the `history` builtin?

If several Bash sessions are open with commands running at overlapping time intervals but in separate sessions, then when each session is closed the history is appended to the end of the history file, resulting in an out-of-order history. The builtin doesn't do any sorting, so it isn't a very nice experience to audit history, and it doesn't output as tab-separated values so it isn't suitable for passing to tools like `cut`.

## Build

```
$ cargo build --release

$ target/release/hs

1  2021-07-02 23:47:41     git status
2  2021-07-02 23:47:42     git add .
3  2021-07-02 23:47:48     git commit -m 'Initial commit'
4  2021-07-02 23:47:49     git status
5  2021-07-02 23:48:05     git add README.md LICENCE.md
6  2021-07-02 23:48:14     git commit -m 'docs: Add README and LICENCE'
7  2021-07-02 23:48:16     git status
```

## CLI

```
Usage:
    hs [options] [ --file PATH ]

Parse timestamped Bash history and output it sorted.

Options:
    -f, --file PATH    Timestamped Bash history file
    -h, --help         Show help
```

## Efficiency

This tool was written (and rewritten) in several languages over time, culminating in this final Rust version. Due to the natural growth of a history file each implementation started to suffer from speed issues and a better solution was needed to maintain the desired performance.

This implementation makes no additional allocations beyond the initial one to read in the entire history file into memory. There is still room for improvement as the date formatting code takes up approximately 30% of execution time.

Benchmarked with a ~467,000-line history file that results in ~200,000 lines of output:

```
$ hyperfine hs

Benchmark #1: hs
  Time (mean ± σ):     181.7 ms ±  15.4 ms    [User: 170.1 ms, System: 11.4 ms]
  Range (min … max):   167.2 ms … 205.5 ms    16 runs
```

## Contributions

Open an [issue](https://github.com/crdx/hs/issues) or send a [pull request](https://github.com/crdx/hs/pulls).

## Licence

[MIT](LICENCE.md).
