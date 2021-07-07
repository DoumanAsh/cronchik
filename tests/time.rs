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

    let schedule = CronSchedule::parse_str("0 20 * MAR FRI").unwrap();
    assert_eq!(schedule.next_time_from(time).date(), time::date!(2019-03-01));

    let schedule = CronSchedule::parse_str("0 20 * * SAT").unwrap();
    assert_eq!(schedule.next_time_from(time).date(), time::date!(2019-02-02));

    let schedule = CronSchedule::parse_str("0 20 * * MON").unwrap();
    assert_eq!(schedule.next_time_from(time).date(), time::date!(2019-02-04));

    let schedule = CronSchedule::parse_str("0 20 * * TUE").unwrap();
    assert_eq!(schedule.next_time_from(time).date(), time::date!(2019-02-05));

    let schedule = CronSchedule::parse_str("0 20 * * WED").unwrap();
    assert_eq!(schedule.next_time_from(time).date(), time::date!(2019-02-06));

    //fits time's Date
    let schedule = CronSchedule::parse_str("0 20 * * THU").unwrap();
    assert_eq!(schedule.next_time_from(time).date(), time::date!(2019-01-31));
}

#[test]
fn should_schedule_every_sunday() {
    let time = time::date!(2019-01-31).midnight().assume_utc();
    let schedule = CronSchedule::parse_str(cronchik::WEEKLY).unwrap();
    assert_eq!(schedule.minutes().len(), 1);
    assert_eq!(schedule.hours().len(), 1);
    assert_eq!(schedule.days_of_week().len(), 1);

    assert_eq!(schedule.days_of_week().len(), 1);
    let mut prev = schedule.next_time_from(time);
    for _ in 0..10 {
        let next = schedule.next_time_from(prev);
        assert_ne!(prev.date(), next.date());
        prev = next;
    }
}

#[test]
fn should_schedule_every_hour() {
    let time = time::date!(2019-01-31).midnight().assume_utc();
    let schedule = CronSchedule::parse_str(cronchik::HOURLY).unwrap();
    assert_eq!(schedule.minutes().len(), 1);
    assert_eq!(schedule.hours().len(), 24);

    let mut prev = schedule.next_time_from(time);
    for _ in 0..10 {
        let next = schedule.next_time_from(prev);
        assert_eq!(prev.date(), next.date());
        assert_ne!(prev.time(), next.time());
        assert_eq!(next - prev, time::Duration::hour());
        prev = next;
    }
}

#[test]
fn should_schedule_every_month() {
    let time = time::date!(2019-01-31).midnight().assume_utc();
    let schedule = CronSchedule::parse_str(cronchik::MONTHLY).unwrap();
    assert_eq!(schedule.minutes().len(), 1);
    assert_eq!(schedule.hours().len(), 1);
    assert_eq!(schedule.days_of_month().len(), 1);

    let mut prev = schedule.next_time_from(time);
    for _ in 0..10 {
        let next = schedule.next_time_from(prev);

        assert_eq!(prev.date().year(), next.date().year());
        assert_eq!(prev.date().day(), next.date().day());
        assert_eq!(prev.date().month() + 1, next.date().month());

        prev = next;
    }
}

#[test]
fn should_schedule_every_year() {
    let time = time::date!(2019-01-31).midnight().assume_utc();
    let schedule = CronSchedule::parse_str(cronchik::YEARLY).unwrap();
    assert_eq!(schedule.minutes().len(), 1);
    assert_eq!(schedule.hours().len(), 1);
    assert_eq!(schedule.days_of_month().len(), 1);
    assert_eq!(schedule.months().len(), 1);

    let mut prev = schedule.next_time_from(time);
    for _ in 0..10 {
        let next = schedule.next_time_from(prev);

        assert_eq!(prev.date().year() + 1, next.date().year());
        assert_eq!(prev.date().day(), next.date().day());
        assert_eq!(prev.date().month(), next.date().month());

        prev = next;
    }
}

#[test]
fn should_pass_100_iterations() {
    let expected_time = time::OffsetDateTime::from_unix_timestamp(1_590_274_800);
    let mut time = time::OffsetDateTime::from_unix_timestamp(1_573_239_864);

    for _ in 0..=100 {
        time = cronchik::parse_cron_from_time("0 23 */2 * *", time).unwrap()
    }

    assert_eq!(time, expected_time);
}
