# ff

This is a tool I wrote for working on [flox][flox].

## Usage

```
Usage: ff <COMMAND>

Commands:
  build  Build artifacts [aliases: b]
  test   Run (parts of) the test suite [aliases: t]
  clean  Remove build artifacts [aliases: c]
  bin    Prints the path to the flox binary
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

The `ff build` command takes shorthands for various components and will build any necessary dependencies e.g. `ff b scripts` will rebuild `flox-activations` as well:

```
pub fn build(args: &BuildArgs) -> Result<()> {
    match args.artifact.as_str() {
        "all" => build_all(args.nix),
        "scripts" => ActivationScripts.build(args.nix),
        "activations" | "act" => FloxActivations.build(args.nix),
        "flox" | "cli" => Flox.build(args.nix),
        "plugins" | "nix-plugins" => NixPlugin.build(args.nix),
        "buildenv" => Buildenv.build(args.nix),
        "package-builder" => PackageBuilder.build(args.nix),
        "watchdog" => Watchdog.build(args.nix),
        "nix" => build_nix_components(),
        _ => anyhow::bail!("unknown artifact: {}", args.artifact),
    }
}
```

The `ff bin` command prints out the absolute path to the `flox` binary in the current repo, which is useful when you want to call it from outside of the devshell or in another terminal tab.

The `ff test` command by default runs the unit tests and then the integration tests.
You can specify running one or the other via the `ff t -u` or `ff t -i` flags.
For the integration tests you can pass arguments to `bats` as long as those arguments don't look like arguments to `ff` (this is slightly broken), so `ff t -i activate.bats` works, but `ff t -i --test-tags foo` doesn't.
You can also specify that you want the integration tests run with a Nix-built `flox` binary instead of the development build via the `ff t -i -n` flag.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[flox]: https://github.com/flox/flox
