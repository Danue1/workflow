macro_rules! snapshot {
    ($( $name:ident -> $source:expr, )+) => {
        $(
            #[test]
            fn $name() {
                let result = crate::DateRule::parse($source);
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
        specifier_13 -> "13",
        specifier_14 -> "14",
        specifier_15 -> "15",
        specifier_16 -> "16",
        specifier_17 -> "17",
        specifier_18 -> "18",
        specifier_19 -> "19",
        specifier_20 -> "20",
        specifier_21 -> "21",
        specifier_22 -> "22",
        specifier_23 -> "23",
        specifier_24 -> "24",
        specifier_25 -> "25",
        specifier_26 -> "26",
        specifier_27 -> "27",
        specifier_28 -> "28",
        specifier_29 -> "29",
        specifier_30 -> "30",
        specifier_31 -> "31",
        specifier_periodic -> "1/2",
        range -> "1-31",
        range_reverse -> "31-1",
        range_periodic -> "1-31/2",
        range_periodic_reverse -> "31-1/2",
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
        specifier__specifier -> "1,2",
        specifier__specifier_periodic -> "1,2/2",
        specifier__range -> "1,2-3",
        specifier__range_periodic -> "1,2-3/2",

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
        empty -> "",
        invalid -> "invalid",
        invalid_periodic -> "invalid/2",
        invalid_range -> "1-",
        invalid_range_periodic -> "1-/2",
        invalid_range_periodic_value -> "1-2/invalid",
        invalid_8 -> "8",
        invalid_8_periodic -> "8/2",
    }
}
