use crate::custom_types::CapacityTestDataPoint;

pub fn send_through_sieve_stack(
    _input_data_points: Vec<CapacityTestDataPoint>,
) -> Vec<CapacityTestDataPoint> {
    vec![]
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
