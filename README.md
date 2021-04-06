# Clevo Fan Control for Linux

Project forked and rewrite to rust, because why not?
Forked from https://github.com/SkyLandTW/clevo-indicator

# Usage

## Build and run
```console
cargo build --release
sudo ./target/release/rusty_clevo_fan [duty_percentage]
```

To use without root privileges:
```console
sudo chown root [path_to_rusty_clevo_fan]
sudo chmod u+s  [path_to_rusty_clevo_fan]
```

## Dependencies:
* libc = "0.2"
* cpuio = "0.3"
