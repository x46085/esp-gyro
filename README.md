# ESP-GYRO
Using an ESP32C3 dev board gyro for measurements with Elodin

## Quickstart
Only tested on MacOS, source instructions here:
https://docs.esp-rs.org/std-training/02_2_software.html

Plug in your ESP32, check for connection:
```bash
system_profiler SPUSBDataType | grep -A 11 "USB JTAG"
```

```bash
brew install llvm libuv
rustup toolchain install nightly-2024-06-30 --component rust-src
cargo install cargo-binstall espflash ldproxy
cargo binstall cargo-espflash
cargo run
```

Check for terminal output:
```bash
I (372) main_task: Started on CPU0
I (372) main_task: Calling app_main()
Device ID SHTC3: 0x47
Device ID ICM42670p: 0x67
TEMP: 25.84 °C | HUM: 38.03 % | GYRO: X= -0.30  Y= 0.49  Z= 0.06
TEMP: 25.79 °C | HUM: 38.01 % | GYRO: X= -0.30  Y= -0.12  Z= 0.30
TEMP: 25.83 °C | HUM: 37.97 % | GYRO: X= -0.30  Y= 0.30  Z= 0.30
TEMP: 25.81 °C | HUM: 37.93 % | GYRO: X= -0.30  Y= 0.79  Z= 0.12
```

