#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

/// This should get an `_address` byte.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Empty {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Empty() {
    assert_eq!(
        ::std::mem::size_of::<Empty>(),
        1usize,
        concat!("Size of: ", stringify!(Empty))
    );
    assert_eq!(
        ::std::mem::align_of::<Empty>(),
        1usize,
        concat!("Alignment of ", stringify!(Empty))
    );
}
/// This should not get an `_address` byte, since each `Empty` gets one, meaning
/// that this object is addressable.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct HasArrayOfEmpty {
    pub empties: [Empty; 10usize],
}
#[test]
fn bindgen_test_layout_HasArrayOfEmpty() {
    assert_eq!(
        ::std::mem::size_of::<HasArrayOfEmpty>(),
        10usize,
        concat!("Size of: ", stringify!(HasArrayOfEmpty))
    );
    assert_eq!(
        ::std::mem::align_of::<HasArrayOfEmpty>(),
        1usize,
        concat!("Alignment of ", stringify!(HasArrayOfEmpty))
    );
    fn test_field_empties() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<HasArrayOfEmpty>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).empties) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(HasArrayOfEmpty),
                "::",
                stringify!(empties)
            )
        );
    }
    test_field_empties();
}
