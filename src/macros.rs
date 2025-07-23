#[macro_export]
macro_rules! packignore {
($($item:literal),* $(,)?) => {
        concat!($(
            $item, "\n",
        )*)
    };
}
