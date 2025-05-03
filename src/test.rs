// src/tests.rs
#[cfg(test)]
mod tests {
    use crate::models::WaterData;

    fn sample_record() -> WaterData {
        WaterData {
            country: "TestLand".into(),
            body: "Lake Test".into(),
            ph: 7.0,
            turbidity: 10.0,
            doxygen: 2.0,
            nitrate: 5.0,
            lead: 3.0,
        }
    }

    #[test]
    fn test_contamination_score() {
        let record = sample_record();
        let score = record.contamination_score();
        assert!((score - 16.0).abs() < f32::EPSILON);
    }
}
