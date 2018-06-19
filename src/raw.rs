// Wrap the "raw" ffi interface in a private library, so that implementers,
// but not users can access it.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {

    // import all of our sox stuff
    use super::super::*;
    use super::*;
    use std::ffi::{CStr, CString};
    use std::ptr::null;

    use std::path::PathBuf;

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

            let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            d.push("data/test.mp3");

            let path = CString::new(d.to_str().unwrap()).unwrap();

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
    fn read_from_file() {
        unsafe {
            // init_sox();
            let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            d.push("data/test.mp3");

            let path = CString::new(d.to_str().unwrap()).unwrap();

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
    fn read_till_eof() {
        unsafe {
            let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            d.push("data/test.mp3");
            let path = CString::new(d.to_str().unwrap()).unwrap();

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

            print!("Final 16 samples: [");
            for b in &buffer[..16] {
                print!("{},", b);
            }
            println!("]");

            // assert_eq!(read_result, 64);

            let close_result = sox_close(open_read_result);

            assert_eq!(close_result, SoxErrorT::Success as i32);
        }
    }
}
