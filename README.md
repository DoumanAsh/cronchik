# cronchik

[![Crates.io](https://img.shields.io/crates/v/cronchik.svg)](https://crates.io/crates/cronchik)
[![Documentation](https://docs.rs/cronchik/badge.svg)](https://docs.rs/crate/cronchik/)
[![Build](https://github.com/DoumanAsh/cronchik/workflows/Rust/badge.svg)](https://github.com/DoumanAsh/cronchik/actions?query=workflow%3ARust)

Cron expression parser.

## Syntax

```
# ┌───────────── minute (0 - 59)
# │ ┌───────────── hour (0 - 23)
# │ │ ┌───────────── day of the month (1 - 31)
# │ │ │ ┌───────────── month (1 - 12)
# │ │ │ │ ┌───────────── day of the week (0 - 6) (Sunday to Saturday)
# │ │ │ │ │
# │ │ │ │ │
# * * * * *
```

## Features

- `serde` - Enables serialization/deserialization.
- `time` - Enables schedule calculation using `time` crate.
