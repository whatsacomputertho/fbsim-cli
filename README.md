# Fbsim CLI

> A Rust-based CLI for simulating american football games, based on the fbsim-core crate

## Overview

A command-line interface built atop the fbsim-core Rust crate which enables users to simulate american football games.

<img src="doc/fbsim-cli-demo.gif" width="75%" alt="Demo of the fbsim CLI">

## Usage

### Game simulation

To simulate a game, execute the following command. Here the `home.json` and `away.json` files are teams as described in the [_team specification_](#team-specification) section.
```sh
fbsim game sim --home home.json --away away.json
```

### Team specification

An example team is given below. Here, the numeric skill level properties MUST be in range `[0, 100]`.

```json
{
    "name": "Null Island Defaults",
    "short_name": "NULL",
    "coach": {
        "risk_taking": 50,
        "run_pass": 50,
        "up_tempo": 50
    },
    "offense": {
        "passing": 50,
        "blocking": 50,
        "rushing": 50,
        "receiving": 50,
        "scrambling": 50,
        "turnovers": 50,
        "field_goals": 50,
        "punting": 50,
        "kickoffs": 50,
        "kick_return_defense": 50
    },
    "defense": {
        "blitzing": 50,
        "rush_defense": 50,
        "pass_defense": 50,
        "coverage": 50,
        "turnovers": 50,
        "kick_returning": 50
    }
}
```
