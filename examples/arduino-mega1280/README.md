
The `Mega1280` board cannot be auto-detected by ravedude, you need to specify the port explicitly.

This can be done in two ways:

* Add the `-P` flag to the ravedude invocation such as in the `.cargo/config.toml`, for example:
```toml 
runner = "ravedude -P /dev/ttyUSB0 -cb 57600 mega1280"
```

* Set the `RAVEDUDE_PORT` environment variable, for example:
```bash
RAVEDUDE_PORT=/dev/ttyUSB0 cargo run --bin mega1280-i2cdetect 
```
