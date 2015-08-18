#![feature(no_std)]
#![feature(asm)]
#![no_std]

#![crate_name = "x86"]
#![crate_type = "lib"]

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate raw_cpuid;

#[cfg(test)]
extern crate std;

#[cfg(not(test))]
mod std {
    pub use core::ops;
    pub use core::option;
}

pub mod io;
pub mod controlregs;
pub mod msr;
pub mod time;
pub mod irq;
pub mod rflags;
pub mod paging;
pub mod segmentation;
pub mod task;
pub mod dtables;
pub mod syscall;
pub mod cpuid {
    pub use raw_cpuid::*;
}