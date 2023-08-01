use crate::custom_types::CapacityTestDataPoint;

pub fn filter_within_20_percent_poa(
    input_data_points: Vec<CapacityTestDataPoint>,
) -> Vec<CapacityTestDataPoint> {
    input_data_points.into_iter().filter(|_x| true).collect()
}
