use crate::types::capacity_test_model_data_point::CapacityTestModelDataPoint;

/// filters out bottom and top 20% (Assumes input data points are already sorted)
pub fn filter_within_60_percent_poa(
    input_data_points: Vec<CapacityTestModelDataPoint>,
) -> Vec<CapacityTestModelDataPoint> {
    let length: f64 = input_data_points.len() as f64;

    input_data_points
        .into_iter()
        .filter(|current_data_point: &CapacityTestModelDataPoint| {
            const TWENTY_PERCENT: f64 = 0.2;

            let bottom_cutoff: f64 = TWENTY_PERCENT * length;
            let top_cutoff: f64 = length * (1 as f64 - TWENTY_PERCENT);

            current_data_point.E_irradiance as f64 > bottom_cutoff
                && (current_data_point.E_irradiance as f64) < top_cutoff
        })
        .collect()
}

#[cfg(test)]
mod filter_within_60_percent_poa_tests {
    use crate::unit_test_mocks::dummy_input_data_point;

    use super::filter_within_60_percent_poa;

    #[test]
    fn empty_returns_empty() {
        let result = filter_within_60_percent_poa(vec![]);
        assert_eq!(result.len(), 0)
    }

    #[test]
    fn filters_out_single_point() {
        let result = filter_within_60_percent_poa(vec![dummy_input_data_point()]);
        assert_eq!(result.len(), 0)
    }

    #[test]
    fn filters_out_two_points_same_poa() {
        let result =
            filter_within_60_percent_poa(vec![dummy_input_data_point(), dummy_input_data_point()]);
        assert_eq!(result.len(), 0)
    }

    #[test]
    fn keeps_first_and_second_from_1_2_3() {
        let mut one = dummy_input_data_point();
        let mut two = dummy_input_data_point();
        let mut three = dummy_input_data_point();

        one.E_irradiance = 1;
        two.E_irradiance = 2;
        three.E_irradiance = 3;

        let result = filter_within_60_percent_poa(vec![one, two, three]);

        let first = result.get(0).unwrap();
        let second = result.get(1).unwrap();

        assert_eq!(result.len(), 2);

        assert_eq!(first.E_irradiance, 1);
        assert_eq!(second.E_irradiance, 2);
    }

    #[test]
    fn keeps_3_through_seven_from_1_2_3_4_5_6_7_8_9_10() {
        let mut one = dummy_input_data_point();
        let mut two = dummy_input_data_point();
        let mut three = dummy_input_data_point();
        let mut four = dummy_input_data_point();
        let mut five = dummy_input_data_point();
        let mut six = dummy_input_data_point();
        let mut seven = dummy_input_data_point();
        let mut eight = dummy_input_data_point();
        let mut nine = dummy_input_data_point();
        let mut ten = dummy_input_data_point();

        one.E_irradiance = 1;
        two.E_irradiance = 2;
        three.E_irradiance = 3;
        four.E_irradiance = 4;
        five.E_irradiance = 5;
        six.E_irradiance = 6;
        seven.E_irradiance = 7;
        eight.E_irradiance = 8;
        nine.E_irradiance = 9;
        ten.E_irradiance = 10;

        let result = filter_within_60_percent_poa(vec![
            one, two, three, four, five, six, seven, eight, nine, ten,
        ]);

        let first = result.get(0).unwrap();
        let second = result.get(1).unwrap();
        let third = result.get(2).unwrap();
        let fourth = result.get(3).unwrap();
        let fifth = result.get(4).unwrap();

        assert_eq!(result.len(), 5);

        assert_eq!(first.E_irradiance, 3);
        assert_eq!(second.E_irradiance, 4);
        assert_eq!(third.E_irradiance, 5);
        assert_eq!(fourth.E_irradiance, 6);
        assert_eq!(fifth.E_irradiance, 7);
    }
}
