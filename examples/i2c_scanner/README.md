# i2c_scanner

## About

This application demonstrates I2C initialization and usage.

The configuration of which I2C bus and pins are used for the scanner is defined
in the [`pins`](./src/pins.rs) module.

## How to run

In this directory, run

    laze build -b bbc-microbit-v2 run

## Example output

    [INFO ] Checking for I2C devices on the bus... (i2c_scanner i2c_scanner/src/main.rs:28)
    [INFO ] Found device at address 25 (i2c_scanner i2c_scanner/src/main.rs:32)
    [INFO ] Found device at address 30 (i2c_scanner i2c_scanner/src/main.rs:32)
    [INFO ] Found device at address 112 (i2c_scanner i2c_scanner/src/main.rs:32)
    [INFO ] Found device at address 113 (i2c_scanner i2c_scanner/src/main.rs:32)
    [INFO ] Found device at address 114 (i2c_scanner i2c_scanner/src/main.rs:32)
    [INFO ] Done checking. Have a great day! (i2c_scanner i2c_scanner/src/main.rs:36)
