macro_rules! rsr {
    ($R:ident) => {
        impl ::sealed::Rsr for $R {
            unsafe fn __rsr(&self) -> u32 {
                let r: u32;
                asm!(concat!("mrs $0,", stringify!($R)) : "=r"(r) : : : "volatile");
                r
            }
        }
    };
}

macro_rules! rsrp {
    ($R:ident) => {
        impl ::sealed::Rsrp for $R {
            unsafe fn __rsrp(&self) -> *const u8 {
                let r: *const u8;
                asm!(concat!("mrs $0,", stringify!($R)) : "=r"(r) : : : "volatile");
                r
            }
        }
    };
}

macro_rules! wsr {
    ($R:ident) => {
        impl ::sealed::Wsr for $R {
            unsafe fn __wsr(&self, value: u32) {
                asm!(concat!("msr ", stringify!($R), ",$0") : : "r"(value) : : "volatile");
            }
        }
    };
}

macro_rules! wsrp {
    ($R:ident) => {
        impl ::sealed::Wsrp for $R {
            unsafe fn __wsrp(&self, value: *const u8) {
                asm!(concat!("msr ", stringify!($R), ",$0") : : "r"(value) : : "volatile");
            }
        }
    };
}

#[cfg(target_feature = "mclass")]
mod v6m;

#[cfg(target_feature = "mclass")]
pub use self::v6m::*;

#[cfg(all(target_feature = "v7", target_feature = "mclass"))]
mod v7m;

#[cfg(all(target_feature = "v7", target_feature = "mclass"))]
pub use self::v7m::*;

#[cfg(not(target_arch = "aarch64"))]
mod aarch32;

#[cfg(not(target_arch = "aarch64"))]
pub use self::aarch32::*;

/// Reads a 32-bit system register
#[inline(always)]
pub unsafe fn __rsr<R>(reg: R) -> u32
where
    R: ::sealed::Rsr,
{
    reg.__rsr()
}

/// Reads a 64-bit system register
#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub unsafe fn __rsr64<R>(reg: R) -> u64
where
    R: ::sealed::Rsr64,
{
    reg.__rsr64()
}

/// Reads a system register containing an address
#[inline(always)]
pub unsafe fn __rsrp<R>(reg: R) -> *const u8
where
    R: ::sealed::Rsrp,
{
    reg.__rsrp()
}

/// Writes a 32-bit system register
#[inline(always)]
pub unsafe fn __wsr<R>(reg: R, value: u32)
where
    R: ::sealed::Wsr,
{
    reg.__wsr(value)
}

/// Writes a 64-bit system register
#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub unsafe fn __wsr64<R>(reg: R, value: u64)
where
    R: ::sealed::Wsr64,
{
    reg.__wsr64(value)
}

/// Writes a system register containing an address
#[inline(always)]
pub unsafe fn __wsrp<R>(reg: R, value: *const u8)
where
    R: ::sealed::Wsrp,
{
    reg.__wsrp(value)
}
