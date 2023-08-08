use std::time::Instant;

use crate::types::{capacity_test_model_data_point::CapacityTestModelDataPoint, month::Month};

// pub const DEFAULT_TIME: Instant = Instant::from("03/21/1991");
pub const DEFAULT_P_POWER: u128 = 1;
pub const DEFAULT_E_IRRADIANCE: u128 = 2;
pub const DEFAULT_TA_AMBIENT_AIR_TEMPERATURE: i128 = 3;
pub const DEFAULT_V_WIND_SPEED: i128 = 4;
pub const DEFAULT_MPPT_IS_BELOW_MAX_OUTPUT: bool = true;
pub const DEFAULT_FSHDBM: i128 = 5;
pub const DEFAULT_FSHDTOP: i128 = 6;
pub const DEFAULT_MONTH: Month = Month::Jan;

pub fn dummy_input_data_point() -> CapacityTestModelDataPoint {
    let now = Instant::now();

    CapacityTestModelDataPoint {
        timestamp: now,
        P_power: DEFAULT_P_POWER,
        E_irradiance: DEFAULT_E_IRRADIANCE,
        Ta_ambient_air_temperature: DEFAULT_TA_AMBIENT_AIR_TEMPERATURE,
        v_wind_speed: DEFAULT_V_WIND_SPEED,
        MPPT_is_below_max_output: DEFAULT_MPPT_IS_BELOW_MAX_OUTPUT,
        FShdBm_shading_something: DEFAULT_FSHDBM,
        FShdTop_shading_something_else: DEFAULT_FSHDTOP,
        month: DEFAULT_MONTH,
    }
}
