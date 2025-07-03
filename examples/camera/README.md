# i2c-scanner

## About

This application demonstrates I2C initialization and usage.

The configuration of which I2C bus and pins are used for the scanner is defined
in the [`pins`](./src/pins.rs) module.

## How to run

In this directory, run

    laze build -b bbc-microbit-v2 run

## Example output

    [INFO ] Checking for I2C devices on the bus... (i2c_scanner i2c-scanner/src/main.rs:21)
    [INFO ] Found device at address 0x19 (i2c_scanner i2c-scanner/src/main.rs:25)
    [INFO ] Found device at address 0x1e (i2c_scanner i2c-scanner/src/main.rs:25)
    [INFO ] Found device at address 0x70 (i2c_scanner i2c-scanner/src/main.rs:25)
    [INFO ] Found device at address 0x72 (i2c_scanner i2c-scanner/src/main.rs:25)
    [INFO ] Done checking. Have a great day! (i2c_scanner i2c-scanner/src/main.rs:29)
