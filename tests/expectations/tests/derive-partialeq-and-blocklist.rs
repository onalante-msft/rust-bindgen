#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub struct BlocklistMe(u8);

/// Because this type contains a blocklisted type, it should not derive
/// PartialEq.
#[repr(C)]
pub struct ShouldNotDerivePartialEq {
    pub a: BlocklistMe,
}
#[test]
fn bindgen_test_layout_ShouldNotDerivePartialEq() {
    assert_eq!(
        ::std::mem::size_of::<ShouldNotDerivePartialEq>(),
        1usize,
        concat!("Size of: ", stringify!(ShouldNotDerivePartialEq))
    );
    assert_eq!(
        ::std::mem::align_of::<ShouldNotDerivePartialEq>(),
        1usize,
        concat!("Alignment of ", stringify!(ShouldNotDerivePartialEq))
    );
    fn test_field_a() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<ShouldNotDerivePartialEq>::uninit(
                    );
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(ShouldNotDerivePartialEq),
                "::",
                stringify!(a)
            )
        );
    }
    test_field_a();
}
impl Default for ShouldNotDerivePartialEq {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
