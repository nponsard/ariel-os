# About

Some nrf's, when time is enabled, need `embassy-nrf/time-driver-rtc1`, some
`embassy-nrf/time-driver-grtc`, some another time driver feature.
We're enabling time with the `time` feature in `ariel-os-nrf`, but need to
choose the correct downstream time driver based on context.
Cargo doesn't allow that, so this crate exists as a bridge.
