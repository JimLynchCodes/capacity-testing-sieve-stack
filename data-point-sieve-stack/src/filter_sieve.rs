use crate::custom_types::CapacityTestDataPoint;

pub fn filter_sieve(input_data_points: Vec<CapacityTestDataPoint>) -> Option<Vec<CapacityTestDataPoint>> {
    None
}

#[cfg(test)]
mod filter_sieve_tests {

    use crate::{filter_sieve::filter_sieve, unit_test_mocks::dummy_input_data_point};

    #[test]
    fn filters_out_everything() {
        assert!(filter_sieve(vec!(dummy_input_data_point())).is_none());
    }
}
