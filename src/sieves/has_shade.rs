use crate::custom_types::CapacityTestDataPoint;

pub fn filters_data_that_has_shade(
    input_data_points: Vec<CapacityTestDataPoint>,
) -> Vec<CapacityTestDataPoint> {
    input_data_points
        .into_iter()
        .filter(|x| return x.has_shade == false)
        .collect()
}
