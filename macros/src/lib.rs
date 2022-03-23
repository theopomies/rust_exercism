#[macro_export]
macro_rules! hashmap {
    () => { ::std::collections::HashMap::new() };
    ($($k:expr => $v:expr),+ $(,)?) => {
        {
            let capacity = $crate::hashmap!(@count $($k),*);
            let mut map = ::std::collections::HashMap::with_capacity(capacity);

            $(
                map.insert($k, $v);
            )*

            map
        }
    };
    (@count $($k:expr),*) => {
        <[()]>::len(&[$($crate::hashmap!(@unit $k)),*])
    };
    (@unit $k:expr) => { () };
}
