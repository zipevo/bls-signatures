use std::ffi::CStr;

use bls_dash_sys::GetLastErrorMsg;

use crate::BlsError;

pub(crate) fn c_err_to_result<T, F>(f: F) -> Result<T, BlsError>
where
    F: FnOnce(&mut bool) -> T,
{
    let mut did_error = false;

    let result = f(&mut did_error);

    if did_error {
        let error_message = unsafe { CStr::from_ptr(GetLastErrorMsg()) };

        Err(BlsError {
            msg: String::from_utf8_lossy(error_message.to_bytes()).into_owned(),
        })
    } else {
        Ok(result)
    }
}
