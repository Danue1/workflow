macro_rules! snapshot {
    ($( $name:ident -> $source:expr, )+) => {
        $(
            #[test]
            fn $name() {
                let result = crate::SecondRule::parse($source);
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
        specifier_9 -> "9",
        specifier_10 -> "10",
        specifier_19 -> "19",
        specifier_20 -> "20",
        specifier_29 -> "29",
        specifier_30 -> "30",
        specifier_39 -> "39",
        specifier_40 -> "40",
        specifier_49 -> "49",
        specifier_50 -> "50",
        specifier_59 -> "59",
        range -> "0-59",
        range_reverse -> "59-0",
        range_periodic -> "0-59/2",
        range_periodic_reverse -> "59-0/2",
    }
}

mod multiple {
    snapshot! {
        all__all -> "*,*",
        all__all_periodic -> "*,*/2",
        all__specifier -> "*,1",
        all__specifier_periodic -> "*,1/2",
        all__range -> "*,1-59",
        all__range_periodic -> "*,1-59/2",

        all_periodic__all -> "*/2,*",
        all_periodic__all_periodic -> "*/2,*/2",
        all_periodic__specifier -> "*/2,1",
        all_periodic__specifier_periodic -> "*/2,1/2",
        all_periodic__range -> "*/2,1-59",
        all_periodic__range_periodic -> "*/2,1-59/2",

        specifier__all -> "1,*",
        specifier__all_periodic -> "1,*/2",
        specifier__specifier -> "1,1",
        specifier__specifier_periodic -> "1,1/2",
        specifier__range -> "1,1-59",
        specifier__range_periodic -> "1,1-59/2",

        specifier_periodic__all -> "1/2,*",
        specifier_periodic__all_periodic -> "1/2,*/2",
        specifier_periodic__specifier -> "1/2,1",
        specifier_periodic__specifier_periodic -> "1/2,1/2",
        specifier_periodic__range -> "1/2,1-59",
        specifier_periodic__range_periodic -> "1/2,1-59/2",

        range__all -> "1-59,*",
        range__all_periodic -> "1-59,*/2",
        range__specifier -> "1-59,1",
        range__specifier_periodic -> "1-59,1/2",
        range__range -> "1-59,1-59",
        range__range_periodic -> "1-59,1-59/2",

        range_periodic__all -> "1-59/2,*",
        range_periodic__all_periodic -> "1-59/2,*/2",
        range_periodic__specifier -> "1-59/2,1",
        range_periodic__specifier_periodic -> "1-59/2,1/2",
        range_periodic__range -> "1-59/2,1-59",
        range_periodic__range_periodic -> "1-59/2,1-59/2",
    }
}
