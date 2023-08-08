use std::time::Instant;

use super::month::Month;


#[derive(Debug)]
pub struct CapacityTestModelDataPoint {
    
    // From the Model ASTM E2848-13 Process Diagram
    
    pub timestamp: Instant,
    pub P_power: u128,
    pub E_irradiance: u128,
    pub Ta_ambient_air_temperature: i128,
    pub v_wind_speed: i128,
    pub MPPT_is_below_max_output: bool,
    pub FShdBm_shading_something: i128, 
    pub FShdTop_shading_something_else: i128, 
    pub month: Month, 

}
