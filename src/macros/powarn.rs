/// Panic or warn depending on features that the binary is compiled with.
///
/// This is used to run the application past errors that should panic in a release version for
/// debugging and development reasons.
///
/// ## Usage
///
/// Compile with the feature `dont_panic` and build using the debug target to change the panics
/// into warnings.
///
/// If you do a `--release` build or don't build with the `--features dont_panic` flag a panic
/// will be thrown.
#[clippy::format_args]
#[macro_export]
macro_rules! powarn {
    ($($arg:tt)+) => ({
        // We should only print warnings when the `dont_panic` feature is enabled and the binary is
        // built in debug mode. When this application is built with the `--release` flag, these
        // errors should not be tolerated.
        if cfg!(all(feature="dont_panic", debug_assertions)) {
            compile_error!("Feature `dont_panic` is not compatible with `--release` build. Fix warnings before releasing.")
        }

        // If `dont_panic` is set, just warn the user.
        if cfg!(feature="dont_panic") {
            use log::warn;
            warn!($($arg)+);
        } else {
            panic!($($arg)+);
        }
    });
}
