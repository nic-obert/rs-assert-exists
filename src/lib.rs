

/// Statically checks if the argument is a valid symbol.
/// 
/// Symbols may be deleted in the code. This macro assures that no function is invalidated because of that.
#[macro_export]
macro_rules! assert_exists {
    ($x:expr) => {
        #[allow(path_statements)]
        const _: () = { $x ; };
    };
}

