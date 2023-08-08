use crate::{
    sieves::{
        not_clipping::filter_for_not_clipping, within_poa_range::filter_within_60_percent_poa,
    },
    types::capacity_test_model_data_point::CapacityTestModelDataPoint,
};

pub fn send_through_sieve_stack(
    _input_data_points: Vec<CapacityTestModelDataPoint>,
) -> Vec<CapacityTestModelDataPoint> {
    let filtered_no_shading = filter_for_not_clipping(_input_data_points);
    let filtered_within_poa_range = filter_within_60_percent_poa(filtered_no_shading);

    // Add more filters as needed...

    filtered_within_poa_range
}

#[cfg(test)]
mod send_through_sieve_stack_tests {

    use crate::{
        send_through_sieve_stack::send_through_sieve_stack, unit_test_mocks::dummy_input_data_point,
    };

    #[test]
    fn filters_out_everything() {
        assert!(send_through_sieve_stack(vec!(dummy_input_data_point())).is_empty());
    }
}
