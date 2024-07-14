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
        specifier_8 -> "8",
        specifier_9 -> "9",
        specifier_10 -> "10",
        specifier_11 -> "11",
        specifier_12 -> "12",
        specifier_jan -> "JAN",
        specifier_feb -> "FEB",
        specifier_mar -> "MAR",
        specifier_apr -> "APR",
        specifier_may -> "MAY",
        specifier_jun -> "JUN",
        specifier_jul -> "JUL",
        specifier_aug -> "AUG",
        specifier_sep -> "SEP",
        specifier_oct -> "OCT",
        specifier_nov -> "NOV",
        specifier_dec -> "DEC",
        specifier_numeric_periodic -> "1/2",
        specifier_named_periodic -> "JAN/2",
        specifier_numeric__specifier_named -> "1-JAN",
        specifier_named__specifier_numeric -> "JAN-1",
        range_numeric -> "1-12",
        range_numeric_reverse -> "12-1",
        range_named -> "JAN-MAR",
        range_named_reverse -> "MAR-JAN",
        range_numeric_periodic -> "1-12/2",
        range_named_periodic -> "JAN-MAR/2",
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
        specifier__specifier -> "1,3",
        specifier__specifier_periodic -> "1,3/2",
        specifier__range -> "1,3-5",
        specifier__range_periodic -> "1,3-5/2",

        specifier_periodic__all -> "1/2,*",
        specifier_periodic__all_periodic -> "1/2,*/2",
        specifier_periodic__specifier -> "1/2,3",
        specifier_periodic__specifier_periodic -> "1/2,3/2",
        specifier_periodic__range -> "1/2,3-5",
        specifier_periodic__range_periodic -> "1/2,3-5/2",

        range__all -> "1-3,*",
        range__all_periodic -> "1-3,*/2",
        range__specifier -> "1-3,5",
        range__specifier_periodic -> "1-3,5/2",
        range__range -> "1-3,5-7",
        range__range_periodic -> "1-3,5-7/2",

        range_periodic__all -> "1-3/2,*",
        range_periodic__all_periodic -> "1-3/2,*/2",
        range_periodic__specifier -> "1-3/2,5",
        range_periodic__specifier_periodic -> "1-3/2,5/2",
        range_periodic__range -> "1-3/2,5-7",
        range_periodic__range_periodic -> "1-3/2,5-7/2",
    }
}

mod error {
    snapshot! {
        error_empty -> "",
        error_numeric_invalid -> "13",
        error_named_invalid -> "INVALID",
        error_numeric_periodic_invalid -> "13/2",
        error_named_periodic_invalid -> "INVALID/2",
        error_range_invalid -> "13-15",
        error_range_numeric_invalid -> "13-MAR",
        error_range_named_invalid -> "JAN-15",
        error_range_periodic_invalid -> "13-15/2",
        error_range_numeric_periodic_invalid -> "13-MAR/2",
        error_range_named_periodic_invalid -> "JAN-15/2",
    }
}
