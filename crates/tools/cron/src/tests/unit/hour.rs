macro_rules! snapshot {
    ($( $name:ident -> $source:expr, )+) => {
        $(
            #[test]
            fn $name() {
                let result = crate::HourRule::parse($source);
                insta::assert_debug_snapshot!(result);
            }
        )+
    };
}

mod single {
    snapshot! {
        all -> "*",
        all_periodic -> "*/2",
        specifier_0 -> "0",
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
        range -> "0-23",
        range_reverse -> "23-0",
        range_periodic -> "0-23/2",
        range_periodic_reverse -> "23-0/2",
    }
}

mod multiple {
    snapshot! {
        all__all -> "*,*",
        all__all_periodic -> "*,*/2",
        all__specifier -> "*,1",
        all__specifier_periodic -> "*,1/2",
        all__range -> "*,1-23",
        all__range_periodic -> "*,1-23/2",

        all_periodic__all -> "*/2,*",
        all_periodic__all_periodic -> "*/2,*/2",
        all_periodic__specifier -> "*/2,1",
        all_periodic__specifier_periodic -> "*/2,1/2",
        all_periodic__range -> "*/2,1-23",
        all_periodic__range_periodic -> "*/2,1-23/2",

        specifier__all -> "1,*",
        specifier__all_periodic -> "1,*/2",
        specifier__specifier -> "1,1",
        specifier__specifier_periodic -> "1,1/2",
        specifier__range -> "1,1-23",
        specifier__range_periodic -> "1,1-23/2",

        specifier_periodic__all -> "1/2,*",
        specifier_periodic__all_periodic -> "1/2,*/2",
        specifier_periodic__specifier -> "1/2,1",
        specifier_periodic__specifier_periodic -> "1/2,1/2",
        specifier_periodic__range -> "1/2,1-23",
        specifier_periodic__range_periodic -> "1/2,1-23/2",

        range__all -> "1-23,*",
        range__all_periodic -> "1-23,*/2",
        range__specifier -> "1-23,1",
        range__specifier_periodic -> "1-23,1/2",
        range__range -> "1-23,1-23",
        range__range_periodic -> "1-23,1-23/2",

        range_periodic__all -> "1-23/2,*",
        range_periodic__all_periodic -> "1-23/2,*/2",
        range_periodic__specifier -> "1-23/2,1",
        range_periodic__specifier_periodic -> "1-23/2,1/2",
        range_periodic__range -> "1-23/2,1-23",
        range_periodic__range_periodic -> "1-23/2,1-23/2",
    }
}
