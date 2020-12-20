# memflow-reclass-plugin

This plugin integrates the [memflow](https://github.com/memflow/memflow) physical memory introspection framework with [ReClass.NET](https://github.com/ReClassNET/ReClass.NET).

The plugin uses the memflow crates internally and also holds caches locally. Any connector that can be used with memflow can also be used with this plugin.

## Compilation

Just run the following command to compile the plugin.

```
cargo build --release
```

The resulting plugin can be found under `./target/release/libmemflow_reclass.so` (or dll on windows).

The plugin as well as the `memflow.toml` file have to be put in the ReClass `/Plugins` folders.

## Usage

After the plugin has been copied to the `./Plugins` folder in ReClass it can be selected as a plugin inside of ReClass.

Make sure you start [ReClass.NET](https://github.com/ReClassNET/ReClass.NET) with the appropiate access rights (e.g. SYS_PTRACE) for the connector you intend to use.

More information on access rights can be found in the [memflow repository](https://github.com/memflow/memflow) or in the specific connector repository.

## Configuration

The configuration file offers the following settings:
- `connector` - the name of the connector to use
- `args` - the argument string passed to the connector, optional
- `parse_sections` - will load section information of the process

Depending on the Connector you use it might be useful to disable section parsing as this slow down the ReClass UI.

## Remarks

This plugin is still work in progress and some features might not yet work as expected.

## License

Licensed under MIT License, see [LICENSE](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
