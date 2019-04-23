pub mod arch;
pub mod util;

mod run;
mod cpu;
mod kvm;
mod vm;

use std::collections::HashMap;
use std::os::raw::c_uint;

use crate::util::{fd, map};

pub struct Kvm(fd::Fd);

pub struct VirtualMachine {
    fd: fd::Fd,
    vcpu_mmap_size: usize,
    multi_addr_space: c_uint,
    mem: HashMap<u16, Vec<map::Map<()>>>,
}

pub struct VirtualCpu {
    fd: fd::Fd,
    run: map::Map<run::Run>,
}

#[derive(Debug)]
pub enum ReasonIo<'a> {
    In { port: u16, data: &'a mut [u8] },
    Out { port: u16, data: &'a [u8] },
}

#[derive(Debug)]
pub enum Reason<'a> {
    Halt,
    Io(ReasonIo<'a>)
}