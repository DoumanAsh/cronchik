macro_rules! assert_test {
    ($ty:ty) => {
        use core::fmt::Write;

        type Type = $ty;
        let result = Type::from_expr("*").unwrap();
        println!("{}: Verify *", stringify!($ty));
        for idx in 1..=((Type::MAX - Type::MIN) as usize) {
            let val: u8 = result[idx].into();
            let prev_val: u8 = result[idx-1].into();
            assert_eq!(val, prev_val + 1);
        }

        println!("{}: Verify parser from MIN to MAX", stringify!($ty));
        for idx in Type::MIN..=Type::MAX {
            let text = format!("{}", idx);
            let result = Type::from_expr(&text).unwrap();
            assert_eq!(result.len(), 1);
            let val: u8 = result[0].into();
            assert_eq!(val, idx);
        }

        println!("{}: Verify unique list", stringify!($ty));
        let mut list = String::new();
        let mut expected_len = 1;
        for idx in Type::MIN..=Type::MAX {
            let _ = write!(&mut list, "{}", idx);
            let result = Type::from_expr(&list).expect("To parse list");
            assert_eq!(result.len(), expected_len, "wrong len");
            expected_len += 1;
            list.push(',');
        }

        println!("{}: Verify list repeats", stringify!($ty));
        let result = Type::from_expr("2,1,2,1").expect("To parse list");
        assert_eq!(result.len(), 2, "wrong len");
        let val: u8  = result[0].into();
        assert_eq!(val, 1);
        let val: u8  = result[1].into();
        assert_eq!(val, 2);

        println!("{}: Verify step by 5", stringify!($ty));
        let result = Type::from_expr("*/5").expect("To parse list");
        for (idx, expected_val) in (Type::MIN..=Type::MAX).step_by(5).enumerate() {
            let val: u8 = result[idx].into();
            assert_eq!(val, expected_val);
        }

        println!("{}: Verify step by 0", stringify!($ty));
        Type::from_expr("*/0").expect_err("Should fail step by 0");
    }
}

#[test]
fn assert_second_parser() {
    use cronchik::Second;
    assert_test!(Second);
}

#[test]
fn assert_minute_parser() {
    use cronchik::Minute;
    assert_test!(Minute);
}

#[test]
fn assert_hour_parser() {
    use cronchik::Hour;
    assert_test!(Hour);
}

#[test]
fn assert_day_parser() {
    use cronchik::Day;
    assert_test!(Day);
}

#[test]
fn assert_month_parser() {
    use cronchik::Month;
    assert_test!(Month);
}
