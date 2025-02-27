#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub mod whatever {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
        pub struct typedef_struct {
            pub foo: ::std::os::raw::c_int,
        }
        #[test]
        fn bindgen_test_layout_typedef_struct() {
            assert_eq!(
                ::std::mem::size_of::<typedef_struct>(),
                4usize,
                concat!("Size of: ", stringify!(typedef_struct))
            );
            assert_eq!(
                ::std::mem::align_of::<typedef_struct>(),
                4usize,
                concat!("Alignment of ", stringify!(typedef_struct))
            );
            fn test_field_foo() {
                assert_eq!(
                    unsafe {
                        let uninit =
                            ::std::mem::MaybeUninit::<typedef_struct>::uninit();
                        let ptr = uninit.as_ptr();
                        ::std::ptr::addr_of!((*ptr).foo) as usize - ptr as usize
                    },
                    0usize,
                    concat!(
                        "Offset of field: ",
                        stringify!(typedef_struct),
                        "::",
                        stringify!(foo)
                    )
                );
            }
            test_field_foo();
        }
        #[repr(u32)]
        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
        pub enum typedef_enum {
            BAR = 1,
        }
    }
    pub mod _bindgen_mod_id_12 {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
        pub struct _bindgen_ty_1 {
            pub foo: ::std::os::raw::c_int,
        }
        #[test]
        fn bindgen_test_layout__bindgen_ty_1() {
            assert_eq!(
                ::std::mem::size_of::<_bindgen_ty_1>(),
                4usize,
                concat!("Size of: ", stringify!(_bindgen_ty_1))
            );
            assert_eq!(
                ::std::mem::align_of::<_bindgen_ty_1>(),
                4usize,
                concat!("Alignment of ", stringify!(_bindgen_ty_1))
            );
            fn test_field_foo() {
                assert_eq!(
                    unsafe {
                        let uninit =
                            ::std::mem::MaybeUninit::<_bindgen_ty_1>::uninit();
                        let ptr = uninit.as_ptr();
                        ::std::ptr::addr_of!((*ptr).foo) as usize - ptr as usize
                    },
                    0usize,
                    concat!(
                        "Offset of field: ",
                        stringify!(_bindgen_ty_1),
                        "::",
                        stringify!(foo)
                    )
                );
            }
            test_field_foo();
        }
        pub type typedef_struct = root::_bindgen_mod_id_12::_bindgen_ty_1;
        pub const _bindgen_mod_id_12_BAR:
            root::_bindgen_mod_id_12::_bindgen_ty_2 = _bindgen_ty_2::BAR;
        #[repr(u32)]
        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
        pub enum _bindgen_ty_2 {
            BAR = 1,
        }
        pub use self::super::super::root::_bindgen_mod_id_12::_bindgen_ty_2 as typedef_enum;
    }
}
