#![cfg(feature = "time")]

use cronchik::CronSchedule;

#[test]
fn should_schedule_on_next_minute() {
    let time = time::date!(2019-01-01).midnight().assume_utc();
    let schedule = CronSchedule::parse_str("1 * * * *").unwrap();

    assert_eq!(schedule.minutes().len(), 1);

    assert_eq!(schedule.next_time_from(time).time(), time::time!(00:01));
}

#[test]
fn should_schedule_on_overflow_minute() {
    let time = time::date!(2019-01-01).midnight().assume_utc() + time::Duration::minute();
    let schedule = CronSchedule::parse_str("1 * * * *").unwrap();

    assert_eq!(schedule.minutes().len(), 1);

    assert_eq!(schedule.next_time_from(time).time(), time::time!(01:01));
}

#[test]
fn should_schedule_on_overflow_hour() {
    let time = time::date!(2019-01-01).midnight().assume_utc() + time::Duration::hour() + time::Duration::minute();
    let schedule = CronSchedule::parse_str("1 1 * * *").unwrap();

    assert_eq!(schedule.minutes().len(), 1);
    assert_eq!(schedule.hours().len(), 1);

    assert_eq!(schedule.next_time_from(time).time(), time::time!(01:01));
    assert_eq!(schedule.next_time_from(time).date(), time::date!(2019-01-02));
}

#[test]
fn should_schedule_on_next_hour_and_minute() {
    let time = time::date!(2019-01-01).midnight().assume_utc();
    let schedule = CronSchedule::parse_str("1 1 * * *").unwrap();

    assert_eq!(schedule.minutes().len(), 1);
    assert_eq!(schedule.hours().len(), 1);

    assert_eq!(schedule.next_time_from(time).time(), time::time!(01:01));
}

#[test]
fn should_schedule_on_next_day_and_hour_and_minute() {
    let time = time::date!(2019-01-01).midnight().assume_utc();
    let schedule = CronSchedule::parse_str("0 20 10 * *").unwrap();

    assert_eq!(schedule.minutes().len(), 1);
    assert_eq!(schedule.hours().len(), 1);
    assert_ne!(schedule.days_of_week().len(), 1);
    assert_eq!(schedule.days_of_month().len(), 1);

    assert_eq!(schedule.next_time_from(time).time(), time::time!(20:00));
    assert_eq!(schedule.next_time_from(time).date(), time::date!(2019-01-10));
}

#[test]
fn should_schedule_on_overflow_day_and_hour_and_minute() {
    let time = time::date!(2019-01-21).midnight().assume_utc();
    let schedule = CronSchedule::parse_str("0 20 10 * *").unwrap();

    assert_eq!(schedule.minutes().len(), 1);
    assert_eq!(schedule.hours().len(), 1);
    assert_ne!(schedule.days_of_week().len(), 1);
    assert_eq!(schedule.days_of_month().len(), 1);

    assert_eq!(schedule.next_time_from(time).time(), time::time!(20:00));
    assert_eq!(schedule.next_time_from(time).date(), time::date!(2019-02-10));
}

#[test]
fn should_schedule_on_next_month_and_day_and_hour_and_minute() {
    let time = time::date!(2019-01-01).midnight().assume_utc();
    let schedule = CronSchedule::parse_str("0 20 10 12 *").unwrap();

    assert_eq!(schedule.minutes().len(), 1);
    assert_eq!(schedule.hours().len(), 1);
    assert_eq!(schedule.months().len(), 1);
    assert_ne!(schedule.days_of_week().len(), 1);
    assert_eq!(schedule.days_of_month().len(), 1);

    assert_eq!(schedule.next_time_from(time).time(), time::time!(20:00));
    assert_eq!(schedule.next_time_from(time).date(), time::date!(2019-12-10));
}

#[test]
fn should_schedule_on_overflow_month_and_day_and_hour_and_minute() {
    let time = time::date!(2019-12-01).midnight().assume_utc();
    let schedule = CronSchedule::parse_str("02 20 12 10 *").unwrap();

    assert_eq!(schedule.minutes().len(), 1);
    assert_eq!(schedule.hours().len(), 1);
    assert_eq!(schedule.months().len(), 1);
    assert_ne!(schedule.days_of_week().len(), 1);
    assert_eq!(schedule.days_of_month().len(), 1);

    assert_eq!(schedule.next_time_from(time).time(), time::time!(20:02));
    assert_eq!(schedule.next_time_from(time).date(), time::date!(2020-10-12));
}

#[test]
fn should_schedule_on_next_day_of_week() {
    let time = time::date!(2019-01-01).midnight().assume_utc();
    let schedule = CronSchedule::parse_str("0 20 * * SAT").unwrap();

    assert_eq!(schedule.minutes().len(), 1);
    assert_eq!(schedule.hours().len(), 1);
    assert_eq!(schedule.days_of_week().len(), 1);
    assert_ne!(schedule.days_of_month().len(), 1);

    assert_eq!(schedule.next_time_from(time).time(), time::time!(20:00));
    assert_eq!(schedule.next_time_from(time).date(), time::date!(2019-01-05));
}

#[test]
fn should_schedule_on_overflow_day_of_week() {
    let time = time::date!(2019-01-31).midnight().assume_utc();
    let schedule = CronSchedule::parse_str("0 20 * * SUN").unwrap();

    assert_eq!(schedule.minutes().len(), 1);
    assert_eq!(schedule.hours().len(), 1);
    assert_eq!(schedule.days_of_week().len(), 1);
    assert_ne!(schedule.days_of_month().len(), 1);

    assert_eq!(schedule.next_time_from(time).time(), time::time!(20:00));
    assert_eq!(schedule.next_time_from(time).date(), time::date!(2019-02-03));

    let schedule = CronSchedule::parse_str("0 20 * * FRI").unwrap();
    assert_eq!(schedule.next_time_from(time).date(), time::date!(2019-02-01));

    let schedule = CronSchedule::parse_str("0 20 * * SAT").unwrap();
    assert_eq!(schedule.next_time_from(time).date(), time::date!(2019-02-02));

    let schedule = CronSchedule::parse_str("0 20 * * MON").unwrap();
    assert_eq!(schedule.next_time_from(time).date(), time::date!(2019-02-04));
}

#[test]
fn should_pass_100_iterations() {
    let expected_time = time::OffsetDateTime::from_unix_timestamp(1_590_274_800);
    let mut time = time::OffsetDateTime::from_unix_timestamp(1_573_239_864);
    let schedule = CronSchedule::parse_str("0 23 */2 * *").unwrap();

    for _ in 0..=100 {
        time = schedule.next_time_from(time);
    }

    assert_eq!(time, expected_time);
}
