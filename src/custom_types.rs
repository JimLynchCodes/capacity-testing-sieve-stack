use std::time::Instant;

#[derive(Debug)]
pub struct CapacityTestDataPoint {
    // "model" prefix means from the PVSYST model
    pub model_ghi: f64,
    pub model_temperatute: f64,
    pub model_wind_speed: f64,
    pub model_correction_factor: u128,
    pub model_timestamp: Instant,

    // "observed" prefix means climate and performance data that comes from the data acquisition system
    pub observed_temperature: f64,
    pub observed_wind_speed: f64,
    pub observed_plane_of_irrandiance: f64,
    pub observed_timestamp: Instant,

    pub has_shade: bool,
}
