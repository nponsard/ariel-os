# BLE keyboard demo

## About

This application is demonstrating how to make a BLE HID keyboard. Pressing the first button on the board will send the letter "a" and the first LED on the board displays the caps lock status.

Part of this code is inspired from [rmk](https://github.com/haobogu/rmk) (MIT license).

## Running

In this directory, run

    laze build -b nrf52840dk run

You should see using [BLE Radar](https://github.com/Semper-Viventem/MetaRadar) a device named "Ariel OS BLE".
