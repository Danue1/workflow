macro_rules! snapshot {
    ($( $name:ident -> $source:expr, )+) => {
        $(
            #[test]
            fn $name() {
                let result = crate::Schedule::parse($source);
                insta::assert_debug_snapshot!(result);
            }
        )+
    };
}

snapshot! {
    secondly -> "@secondly",
    minutely -> "@minutely",
    hourly -> "@hourly",
    daily -> "@daily",
    weekly -> "@weekly",
    sunday -> "@sunday",
    monday -> "@monday",
    tuesday -> "@tuesday",
    wednesday -> "@wednesday",
    thursday -> "@thursday",
    friday -> "@friday",
    saturday -> "@saturday",
    monthly -> "@monthly",
    even_monthly -> "@even-monthly",
    odd_monthly -> "@odd-monthly",
    yearly -> "@yearly",
    annually -> "@annually",
    even_yearly -> "@even-yearly",
    odd_yearly -> "@odd-yearly",
    leap_yearly -> "@leap-yearly",
}
