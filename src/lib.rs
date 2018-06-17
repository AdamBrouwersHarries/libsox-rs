pub mod format;

// import the "raw" library interface
mod raw;
use raw::*;


/// Sox error enum
/// Function succeeded = 0
/// End Of File or other error = -1
/// Invalid Audio Header = 2000
/// Unsupported data format = 2001
/// Can't alloc memory = 2002
/// Operation not permitted = 2003
/// Operation not supported = 2004
/// Invalid argument = 2005
#[derive(Clone, Copy)]
pub enum SoxErrorT {
    Success = sox_error_t_SOX_SUCCESS as isize,
    EndOfFile = sox_error_t_SOX_EOF as isize,
    InvalidAudioHeader = sox_error_t_SOX_EHDR as isize,
    UnsupportedDataFormat = sox_error_t_SOX_EFMT as isize,
    CannotAllocMemory = sox_error_t_SOX_ENOMEM as isize,
    OperationNotPermitted = sox_error_t_SOX_EPERM as isize,
    OperationNotSupported = sox_error_t_SOX_ENOTSUP as isize,
    InvalidArgument = sox_error_t_SOX_EINVAL as isize,
}

pub fn init_sox() -> () {
    unsafe {
        sox_init();
        sox_format_init();
    }
}

#[cfg(test)]
mod tests {

    // import all of our sox stuff
    use super::*;
    use std::ffi::{CStr, CString};
    use std::ptr::null;

    #[test]
    fn init_and_close() {
        unsafe {
            let sox_format_init_result = sox_format_init();

            assert_eq!(sox_format_init_result, SoxErrorT::Success as i32);

            let sox_init_result = sox_init();

            assert_eq!(sox_init_result, SoxErrorT::Success as i32);

            let sox_quit_result = sox_quit();

            assert_eq!(sox_quit_result, SoxErrorT::Success as i32);
        }
    }

    #[test]
    fn open_file_for_reading() {
        unsafe {
            // init_sox();

            let path = CString::new("data/test.mp3").unwrap();

            let open_read_result = sox_open_read(path.as_ptr(), null(), null(), null());

            println!("Open read result: {:?}", open_read_result);

            let filetype = CStr::from_ptr((*open_read_result).filetype)
                .to_str()
                .unwrap();

            println!("Filetype: {}", filetype);

            assert_eq!(filetype, "mp3");

            let close_result = sox_close(open_read_result);

            assert_eq!(close_result, SoxErrorT::Success as i32);

            // sox_quit();
        }
    }

    #[test]
    // #[ignore]
    fn read_from_file() {
        unsafe {
            // init_sox();

            let path = CString::new("data/test.mp3").unwrap();

            let open_read_result = sox_open_read(path.as_ptr(), null(), null(), null());

            println!("Open read result: {:?}", open_read_result);

            let filetype = CStr::from_ptr((*open_read_result).filetype)
                .to_str()
                .unwrap();

            println!("Filetype: {}", filetype);

            assert_eq!(filetype, "mp3");

            let close_result = sox_close(open_read_result);

            assert_eq!(close_result, SoxErrorT::Success as i32);

            // sox_quit();
        }
    }
}
