macro_rules! snapshot {
    ($( $name:ident -> $source:expr, )+) => {
        $(
            #[test]
            fn $name() {
                let result = crate::MonthRule::parse($source);
                insta::assert_debug_snapshot!(result);
            }
        )+
    };
}

mod single {
    snapshot! {
        all -> "*",
        all_periodic -> "*/2",
        specifier_1 -> "1",
        specifier_2 -> "2",
        specifier_3 -> "3",
        specifier_4 -> "4",
        specifier_5 -> "5",
        specifier_6 -> "6",
        specifier_7 -> "7",
        specifier_sun -> "SUN",
        specifier_mon -> "MON",
        specifier_tue -> "TUE",
        specifier_wed -> "WED",
        specifier_thu -> "THU",
        specifier_fri -> "FRI",
        specifier_sat -> "SAT",
        specifier_numeric_periodic -> "1/2",
        specifier_named_periodic -> "SUN/2",
        specifier_numeric__specifier_named -> "1-SUN",
        specifier_named__specifier_numeric -> "SUN-1",
        range_numeric -> "1-7",
        range_numeric_reverse -> "7-1",
        range_named -> "SUN-SAT",
        range_named_reverse -> "SAT-SUN",
        range_numeric_periodic -> "1-7/2",
        range_named_periodic -> "SUN-SAT/2",
    }
}

mod multiple {
    snapshot! {
        all__all -> "*,*",
        all__all_periodic -> "*,*/2",
        all__specifier -> "*,1",
        all__specifier_periodic -> "*,1/2",
        all__range -> "*,1-3",
        all__range_periodic -> "*,1-3/2",

        all_periodic__all -> "*/2,*",
        all_periodic__all_periodic -> "*/2,*/2",
        all_periodic__specifier -> "*/2,1",
        all_periodic__specifier_periodic -> "*/2,1/2",
        all_periodic__range -> "*/2,1-3",
        all_periodic__range_periodic -> "*/2,1-3/2",

        specifier__all -> "1,*",
        specifier__all_periodic -> "1,*/2",
        specifier__specifier -> "1,1",
        specifier__specifier_periodic -> "1,1/2",
        specifier__range -> "1,1-3",
        specifier__range_periodic -> "1,1-3/2",

        specifier_periodic__all -> "1/2,*",
        specifier_periodic__all_periodic -> "1/2,*/2",
        specifier_periodic__specifier -> "1/2,1",
        specifier_periodic__specifier_periodic -> "1/2,1/2",
        specifier_periodic__range -> "1/2,1-3",
        specifier_periodic__range_periodic -> "1/2,1-3/2",

        range__all -> "1-3,*",
        range__all_periodic -> "1-3,*/2",
        range__specifier -> "1-3,1",
        range__specifier_periodic -> "1-3,1/2",
        range__range -> "1-3,1-3",
        range__range_periodic -> "1-3,1-3/2",

        range_periodic__all -> "1-3/2,*",
        range_periodic__all_periodic -> "1-3/2,*/2",
        range_periodic__specifier -> "1-3/2,1",
        range_periodic__specifier_periodic -> "1-3/2,1/2",
        range_periodic__range -> "1-3/2,1-3",
        range_periodic__range_periodic -> "1-3/2,1-3/2",
    }
}

mod error {
    snapshot! {
        error_empty -> "",
        error_invalid -> "0",
        error_numeric_invalid -> "8",
        error_numeric_periodic_invalid -> "1/8",
        error_named_invalid -> "SUNN",
        error_named_periodic_invalid -> "SUN/8",
        error_numeric__named_invalid -> "1-SUNN",
        error_range_invalid -> "1-8",
        error_range_periodic_invalid -> "1-3/8",
        error_named__named_invalid -> "SUN-SUNN",
        error_named__named_periodic_invalid -> "SUN-SUN/8",
        error_range__range_invalid -> "1-3,1-8",
        error_range__range_periodic_invalid -> "1-3,1-3/8",
    }
}
