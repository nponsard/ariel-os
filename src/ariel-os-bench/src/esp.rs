use esp_hal::timer::systimer::{SystemTimer, Unit};

use crate::Error;

#[allow(clippy::cast_possible_truncation)]
#[allow(missing_docs)]
#[allow(clippy::missing_errors_doc)]
pub fn benchmark<F: FnMut() -> ()>(iterations: usize, mut f: F) -> Result<usize, Error> {
    let before = SystemTimer::unit_value(Unit::Unit0);

    for _ in 0..iterations {
        f();
    }

    SystemTimer::unit_value(Unit::Unit0)
        .checked_sub(before)
        .map(|total| total as usize / iterations)
        .ok_or(Error::SystemTimerWrapped)
}
