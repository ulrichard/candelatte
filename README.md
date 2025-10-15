# Candelatte 🛥🔋☕

> #### ⚠️ Alpha Warning! ⚠️
> This Rust crate is still in alpha stage. It is something I quickly put together if anyone needed it. I'm aiming to work on it as I need more features.

Currently it can only convert trips downloaded from the [Candela](https://github.com/CandelaSpeedBoat) WebApp (https://app.candela.com/trips) from xlsx format to gpx.
In the future I plan to add features to query the Candela API used in the WebApp.
Candelatte aims to be both a CLI and a Rust crate for interacting with the Candela API.
This part was inspired by Teslatte (https://github.com/gak/teslatte)


## CLI

There is a CLI that can be used to interact with the API. Example:

```bash
$ candelatte ~/Downloads/boat_data_2025-08-25_09-33-39_2025-08-24 15_13_03_trips_C8-S61.xlsx
file written to "/home/richi/Downloads/boat_data_2025-08-25_09-33-39_2025-08-24 15_13_03_trips_C8-S61.gpx"
```

## License

Licensed under the GPL

## Minimum Supported Rust Version (MSRV)

This library should always compile with Rust **1.85**.
