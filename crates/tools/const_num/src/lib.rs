#[macro_export]
macro_rules! numeric {
    ($( $ident:ident -> $ty:ty, )+) => {
        $(
            pub const fn $ident(source: &str) -> $ty {
                let bytes = source.as_bytes();
                let mut result = 0;
                let mut index = 0;
                while index < bytes.len() {
                    result = result * 10 + (bytes[index] - b'0') as $ty;
                    index += 1;
                }
                result
            }
        )+
    };
}

numeric! {
    u8 -> u8,
    u16 -> u16,
    u32 -> u32,
    u64 -> u64,
    u128 -> u128,
    usize -> usize,

    i8 -> i8,
    i16 -> i16,
    i32 -> i32,
    i64 -> i64,
    i128 -> i128,
    isize -> isize,
}
