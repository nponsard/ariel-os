# tcp-client

## About

This application is testing basic
[Embassy](https://github.com/embassy-rs/embassy) _networking_ usage with Ariel OS.

## How to run

In this directory, run

    laze build -b rpi-pico-w run

The application will try to connect to [tcpbin.com](https://tcpbin.com/), a simple echo server using TCP.

Look [here](../README.md#networking) for more information about network configuration.

If everything goes well, you should see the server's response:

    [INFO ] txd: Hello world!
