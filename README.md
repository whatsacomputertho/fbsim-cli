# Fbsim CLI

> A Rust-based CLI for simulating american football games, based on the fbsim-core crate

## Overview

A command-line interface built atop the fbsim-core Rust crate which enables users to simulate american football games.

## Usage

### Game simulation

Create two files, one specifying the away team and another specifying the home team, structured like so.  Here, the `offense_overall` and `defense_overall` properties MUST be in range `[0, 100]`.

```json
{
    "name": "Team Name",
    "offense_overall": 50,
    "defense_overall": 50
}
```

Then execute the following command, assuming your files are named `home.json` and `away.json` in the current working directory.
```sh
fbsim game sim --home home.json --away away.json
```
