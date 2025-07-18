/// Constructs the hardware random number generator (RNG).
pub fn construct_rng(_peripherals: &mut crate::OptionalPeripherals) {
    ariel_os_random::construct_rng(rand::rngs::OsRng::default());
}
