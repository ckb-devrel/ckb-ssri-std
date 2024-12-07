#[cfg(target_arch = "riscv64")]
use core::arch::asm;

use ckb_std::{ckb_constants::SYS_VM_VERSION, debug, error::SysError};


pub const SYS_FIND_OUT_POINT_BY_TYPE: u64 = 2277;
pub const SYS_FIND_CELL_BY_OUT_POINT: u64 = 2287;
pub const SYS_FIND_CELL_DATA_BY_OUT_POINT: u64 = 2297;

#[cfg(target_arch = "riscv64")]
#[allow(clippy::too_many_arguments)]
pub unsafe fn syscall(
    mut a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
    a7: u64,
) -> u64 {
    asm!(
        "ecall",
        inout("a0") a0,
        in("a1") a1,
        in("a2") a2,
        in("a3") a3,
        in("a4") a4,
        in("a5") a5,
        in("a6") a6,
        in("a7") a7
    );
    a0
}

#[cfg(not(target_arch = "riscv64"))]
#[allow(clippy::too_many_arguments)]
pub unsafe fn syscall(
    _a0: u64,
    _a1: u64,
    _a2: u64,
    _a3: u64,
    _a4: u64,
    _a5: u64,
    _a6: u64,
    _a7: u64,
) -> u64 {
    u64::MAX
}

pub fn vm_version() -> u64 {
    unsafe { syscall(0, 0, 0, 0, 0, 0, 0, SYS_VM_VERSION) }
}



/// Load data
/// Return data length or syscall error
fn syscall_load(
    buf_ptr: *mut u8,
    len: usize,
    a2: usize,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
    syscall_num: u64,
) -> Result<usize, SysError> {
    let mut actual_data_len = len;
    let len_ptr: *mut usize = &mut actual_data_len;
    let ret = unsafe {
        syscall(
            buf_ptr as u64,
            len_ptr as u64,
            a2 as u64,
            a3,
            a4,
            a5,
            a6,
            syscall_num,
        )
    };
    build_syscall_result(ret, len, actual_data_len)
}

fn build_syscall_result(
    errno: u64,
    load_len: usize,
    actual_data_len: usize,
) -> Result<usize, SysError> {
    use SysError::*;

    match errno {
        0 => {
            if actual_data_len > load_len {
                return Err(LengthNotEnough(actual_data_len));
            }
            Ok(actual_data_len)
        }
        1 => Err(IndexOutOfBound),
        2 => Err(ItemMissing),
        _ => Err(Unknown(errno)),
    }
}


/// TODO: Update doc
/// Load cell data, read cell data
///
/// Return the loaded data length or a syscall error
///
/// # Arguments
///
/// * `buf` - a writable buf used to receive the data
/// * `type_script` - the type script used to find the outpoint
pub fn find_out_point_by_type(
    buf: &mut [u8],
    type_script: &[u8],
) -> Result<usize, SysError> {
    syscall_load(
        buf.as_mut_ptr(),
        buf.len(),
        type_script.as_ptr() as usize,
        type_script.len() as u64,
        0,
        0,
        0,
        SYS_FIND_OUT_POINT_BY_TYPE,
    )
}

/// TODO: Update doc
/// Load cell data, read cell data
///
/// Return the loaded data length or a syscall error
///
/// # Arguments
///
/// * `buf` - a writable buf used to receive the data
/// * `out_point` - the out point used to find the cell
pub fn find_cell_by_out_point(
    buf: &mut [u8],
    out_point: &[u8],
) -> Result<usize, SysError> {
    syscall_load(
        buf.as_mut_ptr(),
        buf.len(),
        out_point.as_ptr() as usize,
        0,
        0,
        0,
        0,
        SYS_FIND_CELL_BY_OUT_POINT,
    )
}

/// TODO: Update doc
/// Load cell data, read cell data
///
/// Return the loaded data length or a syscall error
///
/// # Arguments
///
/// * `buf` - a writable buf used to receive the data
/// * `out_point` - the out point used to find the cell data
pub fn find_cell_data_by_out_point(
    buf: &mut [u8],
    out_point: &[u8],
) -> Result<usize, SysError> {
    syscall_load(
        buf.as_mut_ptr(),
        buf.len(),
        out_point.as_ptr() as usize,
        0,
        0,
        0,
        0,
        SYS_FIND_CELL_DATA_BY_OUT_POINT,
    )
}

