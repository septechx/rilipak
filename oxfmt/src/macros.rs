pub mod macros_ {
    #[macro_export]
    macro_rules! structure {
    ($($field:expr),* $(,)?) => {{
        let mut fields = std::collections::HashMap::new();
        let mut index = 0;
        $(
            fields.insert(index, $field);
            index += 1;
        )*
        oxfmt::Structure { fields }
    }};
    }

    #[macro_export]
    macro_rules! construct {
        // ident, [ident : type]* [,]?
        ($fields:ident, $($fname:ident : $ftype:ty),* $(,)?) => {{
            $(
                let $fname = $fields
                    .remove(0)
                    .downcast::<$ftype>()
                    .map_err(|_|
                        anyhow::anyhow!("expected {} for field {}", stringify!($fsrc), stringify!($fname))
                    )?
            )*
            Ok(Self {
                $(
                    $fname,
                )*
            })
        }};

        // ident, [ident : type # type]* [,]?
        ($fields:ident, $($fname:ident : $ftype:ty as $fsrc:ty),* $(,)?) => {{
            $(
                let $fname: $ftype = {
                    let src = $fields
                        .remove(0)
                        .downcast::<$fsrc>()
                        .map_err(|_|
                            anyhow::anyhow!("expected {} for field {}", stringify!($fsrc), stringify!($fname))
                        )?;
                    <$ftype>::from(*src)
                };
            )*
            Ok(Self {
                $(
                    $fname,
                )*
            })
        }}
    }
}
