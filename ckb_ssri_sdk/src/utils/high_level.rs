use crate::utils::syscalls;
use ckb_std::{ckb_types::{packed::{CellOutput, CellOutputReader, OutPoint, OutPointReader, Script}, prelude::*}, error::SysError, high_level::BUF_SIZE};
use alloc::vec::Vec;
use alloc::vec;

/// Common method to fully load data from syscall
fn load_data<F: Fn(&mut [u8], usize) -> Result<usize, SysError>>(
    syscall: F,
) -> Result<Vec<u8>, SysError> {
    let mut buf = [0u8; BUF_SIZE];
    match syscall(&mut buf, 0) {
        Ok(len) => Ok(buf[..len].to_vec()),
        Err(SysError::LengthNotEnough(actual_size)) => {
            let mut data = vec![0; actual_size];
            let loaded_len = buf.len();
            data[..loaded_len].copy_from_slice(&buf);
            let len = syscall(&mut data[loaded_len..], loaded_len)?;
            debug_assert_eq!(len + loaded_len, actual_size);
            Ok(data)
        }
        Err(err) => Err(err),
    }
}

/// TODO: Update doc
///
/// Return the cell or a syscall error
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let out_point = find_out_point_by_type(type_script).unwrap();
/// ```
///
/// **Note:** This function can panic if the underlying data is too large,
/// potentially causing an out-of-memory error.
pub fn find_out_point_by_type(type_script: Script) -> Result<OutPoint, SysError> {
    let mut data = [0u8; OutPoint::TOTAL_SIZE];
    syscalls::find_out_point_by_type(&mut data, &type_script.as_slice())?;
    match OutPointReader::verify(&data, false) {
        Ok(()) => Ok(OutPoint::new_unchecked(data.to_vec().into())),
        Err(_err) => Err(SysError::Encoding),
    }
}

/// TODO: Update doc
///
/// Return the cell or a syscall error
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let cell_output = load_cell(0, Source::Input).unwrap();
/// ```
///
/// **Note:** This function can panic if the underlying data is too large,
/// potentially causing an out-of-memory error.
pub fn find_cell_by_out_point(out_point: OutPoint) -> Result<CellOutput, SysError> {
    let data =
        load_data(|buf, offset| syscalls::find_cell_by_out_point(buf, out_point.as_slice()))?;

    match CellOutputReader::verify(&data, false) {
        Ok(()) => Ok(CellOutput::new_unchecked(data.into())),
        Err(_err) => Err(SysError::Encoding),
    }
}

/// TODO: Update doc
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let data = load_cell_data(index, source).unwrap();
/// ```
///
/// **Note:** This function can panic if the underlying data is too large,
/// potentially causing an out-of-memory error.
pub fn find_cell_data_by_out_point(out_point: OutPoint) -> Result<Vec<u8>, SysError> {
    load_data(|buf, offset| syscalls::find_cell_data_by_out_point(buf, out_point.as_slice()))
}
