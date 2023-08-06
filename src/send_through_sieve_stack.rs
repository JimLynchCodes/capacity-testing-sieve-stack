use crate::{custom_types::CapacityTestDataPoint, sieves::poa_range::filter_within_20_percent_poa};


pub fn send_through_sieve_stack(
    _input_data_points: Vec<CapacityTestDataPoint>,
) -> Vec<CapacityTestDataPoint> {
 
    let filtered_within_poa_range = filter_within_20_percent_poa(_input_data_points);
    // let filtered_no_shading = filter_for_no_shading(filtered_within_poa_range);
    
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
