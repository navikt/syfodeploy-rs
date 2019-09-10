# syfodeploy-rs

`syfodeploy`, but in Rust!

## Installation

Currently, `syfodeploy-rs` is not on crates.io. You may install `syfodeploy-rs` by running `cargo install --path .` from within this repository.

If you have installed Rust via `rustup`, you may also want to have `~/.cargo/.bin` in your path.
```
export PATH="$HOME/.cargo/bin:$PATH"
```

To uninstall, you can run `cargo uninstall syfodeploy`

## Usage

* You can simply invoke `syfodeploy` to trigger a build with the current git repository as well as the current branch
* You may also trigger a build by running `syfodeploy your_project your_branch`, such as `syfodeploy sykefravaer master`
