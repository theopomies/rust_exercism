use time::ext::NumericalDuration;
use time::PrimitiveDateTime as DateTime;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start + 1e9.seconds()
}
