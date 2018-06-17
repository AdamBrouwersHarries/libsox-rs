pub mod encodinginfo;
pub mod format;
pub mod signalinfo;

// import the "raw" library interface
mod raw;
use raw::*;

#[macro_use]
extern crate once;

use std::ffi::{CStr, CString};
use std::ptr::null;

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

/// Initialise the sox library. This function must only be called once, and will panic if called more than once.
pub fn init_sox() -> () {
    assert_has_not_been_called!("The Sox initialisation function must only be called once");
    unsafe {
        sox_init();
        sox_format_init();
    }
}

/// Read a file into a vector of usize elements
pub fn read_file(filename: String) -> Result<Vec<i32>, SoxErrorT> {
    unsafe {
        // create a path from the filename
        let path = CString::new(filename).unwrap();

        // open the file into a structure
        let sox_format = sox_open_read(path.as_ptr(), null(), null(), null());

        // define our sample size - we're going for a big one for speed.
        const samples: usize = 8192;

        // create a buffer for the samples
        let mut buffer: [std::os::raw::c_int; samples] = [0; samples];

        // create a vector into which we store the the buffer contents
        let mut final_vec = Vec::new();

        // initialise an exit flag
        let mut read_result = samples;

        while read_result != (SoxErrorT::EndOfFile as usize) {
            read_result = sox_read(sox_format, buffer.as_mut_ptr(), samples);
            println!("Read {} samples.", read_result);
            // check that we're not done
            if read_result != (SoxErrorT::EndOfFile as usize) {
                // reserve space in the vector, and copy values in
               final_vec.reserve(read_result);
               // copy into it
               final_vec.extend((buffer as [i32; samples]).iter().cloned());
            }
        }


        let close_result = sox_close(sox_format);

        // if close_result as SoxErrorT != SoxErrorT::Success{
        //     return Err(close_result as SoxErrorT);
        // }

        return Ok(final_vec);
    }
}

#[cfg(test)]
mod tests {

    // import all of our sox stuff
    use super::*;

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

            let mut buffer: [std::os::raw::c_int; 64] = [0; 64];

            let read_result = sox_read(open_read_result, buffer.as_mut_ptr(), 64);

            println!("Read {} samples", read_result);

            assert_eq!(read_result, 64);

            let close_result = sox_close(open_read_result);

            assert_eq!(close_result, SoxErrorT::Success as i32);

            // sox_quit();
        }
    }

    #[test]
    // #[ignore]
    fn read_till_eof() {
        unsafe {
            let path = CString::new("data/test.mp3").unwrap();

            let open_read_result = sox_open_read(path.as_ptr(), null(), null(), null());

            println!("Open read result: {:?}", open_read_result);
            println!("Open read result: {:#?}", open_read_result);

            let filetype = CStr::from_ptr((*open_read_result).filetype)
                .to_str()
                .unwrap();

            println!("Filetype: {}", filetype);

            assert_eq!(filetype, "mp3");

            const samples: usize = 8192;

            let mut buffer: [std::os::raw::c_int; samples] = [0; samples];

            let mut read_result = samples;

            let mut sample_count = 0;
            while read_result == samples {
                read_result = sox_read(open_read_result, buffer.as_mut_ptr(), samples);
                sample_count += 1;
                println!("Read {} samples.", read_result);
            }

            println!("Final read result: {}", read_result);
            println!("Read {} individual blocks of samples. ", sample_count);

            // assert_eq!(read_result, 64);

            let close_result = sox_close(open_read_result);

            assert_eq!(close_result, SoxErrorT::Success as i32);
        }
    }
}
