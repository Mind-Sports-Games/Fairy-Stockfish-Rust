#[cxx::bridge]
pub mod ffi {

    unsafe extern "C++" {
        include!("fairystockfish/src/fairystockfishrs.h");

        fn init();

        /// # Examples
        /// ```
        /// assert_eq!("v0.0.5", fairystockfish::ffi::version());
        /// ````
        fn version() -> String;
        fn info();
    }
}
