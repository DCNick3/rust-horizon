use horizon_error::Result as HorizonResult;
use horizon_error::ErrorCode;

use crate::io::Error as StdError;
use crate::io::Result as StdResult;

pub trait HorizonResultExt<T> {
    fn to_std_result(self) -> StdResult<T>;
}

impl<T> HorizonResultExt<T> for HorizonResult<T> {
    fn to_std_result(self) -> StdResult<T> {
        self.map_err(|e: ErrorCode| {
            StdError::from_raw_os_error(e.repr() as i32)
        })
    }
}
