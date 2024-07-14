macro_rules! snapshot {
    ($( $name:ident -> $source:expr, )+) => {
        $(
            #[test]
            fn $name() {
                let result = crate::YearRule::parse($source);
                insta::assert_debug_snapshot!(result);
            }
        )+
    };
}

mod single {
    snapshot! {
        all -> "*",
        all_periodic -> "*/2",
        specifier -> "2024",
        specifier_periodic -> "2024/2",
        range -> "2024-2026",
        range_periodic -> "2024-2026/2",
    }
}

mod multiple {
    snapshot! {
        all__all -> "*,*",
        all__all_periodic -> "*,*/2",
        all__specifier -> "*,2024",
        all__specifier_periodic -> "*,2024/2",
        all__range -> "*,2024-2026",
        all__range_periodic -> "*,2024-2026/2",

        all_periodic__all -> "*/2,*",
        all_periodic__all_periodic -> "*/2,*/2",
        all_periodic__specifier -> "*/2,2024",
        all_periodic__specifier_periodic -> "*/2,2024/2",
        all_periodic__range -> "*/2,2024-2026",
        all_periodic__range_periodic -> "*/2,2024-2026/2",

        specifier__all -> "2024,*",
        specifier__all_periodic -> "2024,*/2",
        specifier__specifier -> "2024,2026",
        specifier__specifier_periodic -> "2024,2026/2",
        specifier__range -> "2024,2026-2028",
        specifier__range_periodic -> "2024,2026-2028/2",

        specifier_periodic__all -> "2024/2,*",
        specifier_periodic__all_periodic -> "2024/2,*/2",
        specifier_periodic__specifier -> "2024/2,2026",
        specifier_periodic__specifier_periodic -> "2024/2,2026/2",
        specifier_periodic__range -> "2024/2,2026-2028",
        specifier_periodic__range_periodic -> "2024/2,2026-2028/2",

        range__all -> "2024-2026,*",
        range__all_periodic -> "2024-2026,*/2",
        range__specifier -> "2024-2026,2028",
        range__specifier_periodic -> "2024-2026,2028/2",
        range__range -> "2024-2026,2028-2030",
        range__range_periodic -> "2024-2026,2028-2030/2",

        range_periodic__all -> "2024-2026/2,*",
        range_periodic__all_periodic -> "2024-2026/2,*/2",
        range_periodic__specifier -> "2024-2026/2,2028",
        range_periodic__specifier_periodic -> "2024-2026/2,2028/2",
        range_periodic__range -> "2024-2026/2,2028-2030",
        range_periodic__range_periodic -> "2024-2026/2,2028-2030/2",
    }
}

mod error {
    snapshot! {
        error_empty -> "",
        error_invalid -> "invalid",
        error_less_than_1970 -> "1969",
        error_greater_than_2199 -> "2200",
        error_invalid_periodic -> "*/0",
        error_invalid_range -> "2024-2022",
    }
}
