#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct NoHash {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_NoHash() {
    assert_eq!(
        ::std::mem::size_of::<NoHash>(),
        1usize,
        concat!("Size of: ", stringify!(NoHash))
    );
    assert_eq!(
        ::std::mem::align_of::<NoHash>(),
        1usize,
        concat!("Alignment of ", stringify!(NoHash))
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct AllowlistMe {
    pub a: NoHash,
}
#[test]
fn bindgen_test_layout_AllowlistMe() {
    assert_eq!(
        ::std::mem::size_of::<AllowlistMe>(),
        1usize,
        concat!("Size of: ", stringify!(AllowlistMe))
    );
    assert_eq!(
        ::std::mem::align_of::<AllowlistMe>(),
        1usize,
        concat!("Alignment of ", stringify!(AllowlistMe))
    );
    fn test_field_a() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<AllowlistMe>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(AllowlistMe),
                "::",
                stringify!(a)
            )
        );
    }
    test_field_a();
}
