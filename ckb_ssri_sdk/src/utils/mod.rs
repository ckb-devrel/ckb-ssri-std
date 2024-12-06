use ckb_std::debug;
use syscalls::vm_version;

use crate::SSRIError;

pub mod syscalls;
pub mod high_level;

pub fn should_fallback() -> Result<bool, SSRIError> {
  if ckb_std::env::argv().is_empty() {
      debug!("Should fallback!");
      return Ok(true);
  } else {
      if vm_version() != u64::MAX {
          return Err(SSRIError::InvalidVmVersion);
      } else {
          debug!("Should not fallback!");
          return Ok(false);
      }
  }
}
