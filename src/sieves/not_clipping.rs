use crate::types::capacity_test_model_data_point::CapacityTestModelDataPoint;

/// Filters out clipping data points (based on (MPPT_is_below_max_output))
pub fn filter_for_not_clipping(
    input_data_points: Vec<CapacityTestModelDataPoint>,
) -> Vec<CapacityTestModelDataPoint> {
    input_data_points
        .into_iter()
        .filter(|data_point| !(data_point.MPPT_is_below_max_output))
        .collect()
}

#[cfg(test)]
mod filter_filter_for_not_clipping_tests {

    use crate::unit_test_mocks::dummy_input_data_point;

    use super::filter_for_not_clipping;

    #[test]
    fn filters_out_data_with_mppt_true() {
        let mut mock_data_point = dummy_input_data_point();

        mock_data_point.MPPT_is_below_max_output = true;

        let result = filter_for_not_clipping(vec![mock_data_point]);

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn lets_through_data_with_mppt_true() {
        let mut mock_data_point = dummy_input_data_point();

        mock_data_point.MPPT_is_below_max_output = false;

        let result = filter_for_not_clipping(vec![mock_data_point]);

        assert_eq!(result.len(), 1);
    }
}
