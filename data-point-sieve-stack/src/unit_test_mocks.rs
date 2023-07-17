use std::time::{Duration, Instant};

use crate::custom_types::CapacityTestDataPoint;

pub fn dummy_input_data_point() -> CapacityTestDataPoint {
    let now = Instant::now();

    CapacityTestDataPoint {
        model_ghi: 1.0,
        model_temperatute: 2.0,
        model_wind_speed: 3.0,
        model_correction_factor: 4,
        model_timestamp: now + Duration::new(1000, 0),

        observed_temperature: 5.0,
        observed_wind_speed: 6.0,
        observed_plane_of_irrandiance: 7.0,
        observed_timestamp: now + Duration::new(2000, 0),
    }
}
