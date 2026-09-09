# voting_machine

A voting machine prototype in Rust with deliberately embedded backdoors, built
for red-team analysis of election integrity controls. It was built to be
attacked, as part of an exercise in which teams red-teamed each other's
machines.

Project write-up: [Root the (Ballot) Box](https://sha512rma.eth.limo/projects/)

## Do not use this to run an election

The backdoors are intentional and are the whole point of the exercise. The CSV
files at the repository root are synthetic fixtures, not real voter records.

## What it does

Nine modules under `src/` cover the election lifecycle: admin authentication,
voter registration, user signup, ballot creation, ballot saving, opening an
election, casting a vote, closing an election, and tallying. `src/models.rs`
holds the shared types.

State lives in flat CSV files (`elections.csv`, `candidates.csv`, `offices.csv`,
`registered_voters.csv`, `users.csv`, `votes.csv`, `casted_ballots.csv`) with an
append-only `audit_trail.csv`.

## Stack

Rust 2021, with [argon2](https://crates.io/crates/argon2) 0.5.3 for password
hashing, [csv](https://crates.io/crates/csv) 1.3.0 for storage, UUID v4 for
identifiers, and [log4rs](https://crates.io/crates/log4rs) 1.3.0 for logging.

## Run it

```bash
cargo run
```

`assets/session_audit.py` turns the log output into a session audit.
