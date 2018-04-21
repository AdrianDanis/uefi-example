#![feature(lang_items)]
#![feature(compiler_builtins_lib)]
#![feature(duration_extras)]
#![no_std]
#![no_main]

extern crate rlibc;
extern crate compiler_builtins;

extern crate uefi;
extern crate fencedvar;

use uefi::{Handle, Status};
use uefi::table;

#[macro_use]
mod print;
mod panic;
mod intrinsics;

pub use panic::rust_begin_panic;
pub use intrinsics::*;

struct GUIDWrap(uefi::Guid);
impl core::fmt::Display for GUIDWrap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let v = match self.0 {
            uefi::table::cfg::ACPI_GUID => Some("ACPI"),
            uefi::table::cfg::ACPI2_GUID => Some("ACPI2"),
            uefi::table::cfg::PROPERTIES_TABLE_GUID => Some("UEFI Properties"),
            uefi::table::cfg::DEBUG_IMAGE_INFO_GUID => Some("Debug image info"),
            _ => None,
        };
        match v {
            None => write!(f, "unknown GUID {}", self.0),
            Some(s) => write!(f, "{}", s),
        }
    }
}

struct CTEWrap(&'static uefi::table::cfg::ConfigTableEntry);
impl core::fmt::Display for CTEWrap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{} at {:p}", GUIDWrap(self.0.guid), self.0.address as *mut usize)
    }
}

struct MemDescWrap(uefi::table::boot::MemoryDescriptor);
impl core::fmt::Display for MemDescWrap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        // duplicate so we can create temporaries of the packed fields so that rustc shuts the fuck up
        let dup = self.0.clone();
        let ty = dup.ty;
        let att = dup.att;
        write!(f, "{} bytes ({} MiB) at {:p} (0x{:p} virtual) of type {:?} and attributes {:?}", self.0.page_count * 4096, self.0.page_count / 256, self.0.phys_start as usize as *mut usize, self.0.virt_start as usize as *mut usize, ty, att)
    }
}

pub static mut UEFI_SYSTEM_TABLE: Option<&'static table::SystemTable> = None;

#[no_mangle]
pub extern "win64" fn UefiMain(_handle: Handle, st: &'static table::SystemTable) -> Status {
    unsafe {
        UEFI_SYSTEM_TABLE = Some(&st);
    }
    panic::set_panic_action(panic::Action::ShutdownDelay(core::time::Duration::from_secs(1)));
    loader_main(st);
    Status::Success
}

fn loader_main(st: &'static table::SystemTable) -> () {
    println!("Hello world!");
    let mut all_desc_array: [uefi::table::boot::MemoryDescriptor; 32] =
        [uefi::table::boot::MemoryDescriptor::default(); 32];
    let mut temp: [u8; 4096] = [0; 4096];
    let (num_desc, _) = st.boot.get_memory_map(&mut all_desc_array,&mut temp).unwrap();
    let mem_desc = &all_desc_array[..num_desc];
    println!("Found initial memory information:");
    for desc in mem_desc {
        println!("\t{}", MemDescWrap(*desc));
    }
    println!("Have configuration values:");
    for config in st.config_table() {
        println!("\t{}", CTEWrap(config));
    }
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn __chkstk() -> () {
}
