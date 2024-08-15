# ESP-GYRO
Using an ESP32C3 dev board gyro for measurements with Elodin

## Quickstart
Only tested on MacOS, source instructions here:
https://docs.esp-rs.org/std-training/02_2_software.html

1. Copy the cfg.toml.example into cfg.toml and set the SSID & password

2. Plug in your ESP32, check for connection:
```bash
system_profiler SPUSBDataType | grep -A 11 "USB JTAG"
```

3. Install dependencies, build & flash the device
```bash
brew install llvm libuv
rustup toolchain install nightly-2024-06-30 --component rust-src
cargo install cargo-binstall espflash ldproxy
cargo binstall cargo-espflash
cargo run
```

4. Check for terminal output:
```bash
Device ID SHTC3: 0x47
Device ID ICM42670p: 0x67
Sampling, example:
TEMP: 26.83 °C | HUM: 40.21 % | GYRO: X= -0.24  Y= 0.30  Z= -0.06
```

5. Confirm ability to receive the broadcast:
```bash
cd src && rustc udp-radio;
./udp-radio
```
