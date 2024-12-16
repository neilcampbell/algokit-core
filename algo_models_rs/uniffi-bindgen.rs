fn main() {
    #[cfg(feature = "ffi_uniffi")]
    uniffi::uniffi_bindgen_main()
}
