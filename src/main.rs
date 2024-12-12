use std::arch::x86_64::{_xrstor64, _xsavec64};

#[repr(align(64))]
#[derive(Debug)]
struct XsaveArea {
    // max size for 256-bit registers is 800 bytes:
    // see https://software.intel.com/en-us/node/682996
    // max size for 512-bit registers is 2560 bytes:
    // FIXME: add source
    data: [u8; 2560],
}

impl XsaveArea {
    fn new() -> XsaveArea {
        XsaveArea { data: [0; 2560] }
    }
    fn ptr(&mut self) -> *mut u8 {
        self.data.as_mut_ptr()
    }
}

impl PartialEq<XsaveArea> for XsaveArea {
    fn eq(&self, other: &XsaveArea) -> bool {
        for i in 0..self.data.len() {
            // Ignore XSTATE_BV (state-component bitmap) that occupies the first byte of the XSAVE Header
            // (at offset 512 bytes from the start). The value may change, for more information see the following chapter:
            // 13.7 OPERATION OF XSAVE - Intel® 64 and IA-32 Architectures Software Developer’s Manual.
            if i != 512 && self.data[i] != other.data[i] {
                return false;
            }
        }
        true
    }
}

unsafe fn test_xsave64() {
    let m = 0xFFFFFFFFFFFFFFFF_u64; //< all registers
    let mut a = XsaveArea::new();
    let mut b = XsaveArea::new();

    unsafe {
        _xsavec64(a.ptr(), m);
        _xrstor64(a.ptr(), m);
        _xsavec64(b.ptr(), m);
    }

    assert_eq!(a, b);
}

fn main() {
    unsafe {
        test_xsave64();
    }
}
