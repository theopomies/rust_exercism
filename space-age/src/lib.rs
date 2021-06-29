// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(f64);

pub struct OrbitalPeriod(f64);

const EARTH_YEAR_IN_SECONDS: u64 = 31557600;

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s as f64 / EARTH_YEAR_IN_SECONDS as f64)
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: OrbitalPeriod;

    fn years_during(Duration(duration): &Duration) -> f64 {
        let OrbitalPeriod(orbital_period) = Self::ORBITAL_PERIOD;
        duration / orbital_period
    }
}

macro_rules! planet {
    ($name:ident, $orbital_period:expr) => {
        pub struct $name;
        impl Planet for $name {
            const ORBITAL_PERIOD: OrbitalPeriod = $orbital_period;
        }
    };
}

planet!(Mercury, OrbitalPeriod(0.2408467));
planet!(Venus, OrbitalPeriod(0.61519726));
planet!(Earth, OrbitalPeriod(1.0));
planet!(Mars, OrbitalPeriod(1.8808158));
planet!(Jupiter, OrbitalPeriod(11.862615));
planet!(Saturn, OrbitalPeriod(29.447498));
planet!(Uranus, OrbitalPeriod(84.016846));
planet!(Neptune, OrbitalPeriod(164.79132));
