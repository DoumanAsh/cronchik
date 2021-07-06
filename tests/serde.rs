#[cfg(feature = "serde")]
#[test]
fn verify_serialization() {
    use cronchik::CronSchedule;

    let schedule = CronSchedule::parse_str("5 * * * *").unwrap();

    let result = serde_json::to_string(&schedule).unwrap();
    let reverse: CronSchedule = serde_json::from_str(&result).unwrap();
    assert_eq!(reverse, schedule);

    let schedule = CronSchedule::parse_str("5 */5 10/2 MAR-MAY *").unwrap();

    let result = serde_json::to_string(&schedule).unwrap();
    let reverse: CronSchedule = serde_json::from_str(&result).unwrap();
    assert_eq!(reverse, schedule);
}
