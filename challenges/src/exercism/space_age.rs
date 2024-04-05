#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

macro_rules! impl_planet_duration_match {
    (Mercury) => {
        0.2408467
    };
    (Venus) => {
        0.61519726
    };
    (Mars) => {
        1.8808158
    };
    (Jupiter) => {
        11.862615
    };
    (Saturn) => {
        29.447498
    };
    (Uranus) => {
        84.016846
    };
    (Neptune) => {
        164.79132
    };
    (Earth) => {
        1.0
    };
}

macro_rules! impl_planet_duration {
    ($t: ident) => {
        impl Planet for $t {
            fn years_during(d: &Duration) -> f64 {
                let data = impl_planet_duration_match!($t);
                (d.seconds / 365.25 / 24.0 / 60.0 / 60.0 / data * 100.0).round() / 100.0
            }
        }
    };
}
impl From<u64> for Duration {
    fn from(s: u64) -> Duration {
        Duration { seconds: s as f64 }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl_planet_duration!(Mercury);
impl_planet_duration!(Venus);
impl_planet_duration!(Earth);
impl_planet_duration!(Mars);
impl_planet_duration!(Jupiter);
impl_planet_duration!(Saturn);
impl_planet_duration!(Uranus);
impl_planet_duration!(Neptune);
