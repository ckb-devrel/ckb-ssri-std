#[cfg(target_arch = "riscv64")]
use core::arch::asm;

use ckb_std::{ckb_constants::SYS_VM_VERSION, debug, error::SysError};

/// System call number for finding an OutPoint by type script
pub const SYS_FIND_OUT_POINT_BY_TYPE: u64 = 2277;
/// System call number for finding a cell by OutPoint
pub const SYS_FIND_CELL_BY_OUT_POINT: u64 = 2287;
/// System call number for finding cell data by OutPoint
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


/// Find an OutPoint by searching for a specific type script
///
/// Searches for a cell with the given type script and returns its OutPoint.
/// The OutPoint data is written to the provided buffer.
///
/// # Arguments
///
/// * `buf` - A mutable buffer to receive the OutPoint data
/// * `type_script` - The serialized type script to search for
///
/// # Returns
///
/// * `Ok(usize)` - The actual length of the OutPoint data written to the buffer
/// * `Err(SysError)` - A system error if the operation fails
///
/// # Errors
///
/// Returns `SysError::LengthNotEnough` if the buffer is too small to hold the data
/// Returns `SysError::IndexOutOfBound` if the type script is invalid
/// Returns `SysError::ItemMissing` if no matching cell is found
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

/// Find a cell by its OutPoint
///
/// Retrieves cell information using the specified OutPoint.
/// The cell data is written to the provided buffer.
///
/// # Arguments
///
/// * `buf` - A mutable buffer to receive the cell data
/// * `out_point` - The serialized OutPoint identifying the cell to find
///
/// # Returns
///
/// * `Ok(usize)` - The actual length of the cell data written to the buffer
/// * `Err(SysError)` - A system error if the operation fails
///
/// # Errors
///
/// Returns `SysError::LengthNotEnough` if the buffer is too small to hold the data
/// Returns `SysError::IndexOutOfBound` if the OutPoint is invalid
/// Returns `SysError::ItemMissing` if the cell cannot be found
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

/// Find cell data by OutPoint
///
/// Retrieves the data contained in a cell identified by the specified OutPoint.
/// The cell's data is written to the provided buffer.
///
/// # Arguments
///
/// * `buf` - A mutable buffer to receive the cell's data
/// * `out_point` - The serialized OutPoint identifying the cell whose data to retrieve
///
/// # Returns
///
/// * `Ok(usize)` - The actual length of the cell data written to the buffer
/// * `Err(SysError)` - A system error if the operation fails
///
/// # Errors
///
/// Returns `SysError::LengthNotEnough` if the buffer is too small to hold the data
/// Returns `SysError::IndexOutOfBound` if the OutPoint is invalid
/// Returns `SysError::ItemMissing` if the cell cannot be found
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

