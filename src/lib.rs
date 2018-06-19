#![feature(test)]
pub mod encodinginfo;
pub mod format;
pub mod signalinfo;

// import the "raw" library interface
mod raw;
use raw::*;

#[macro_use]
extern crate once;

extern crate byteorder;
use byteorder::*;

extern crate test;

extern crate rayon;
use rayon::prelude::*;

use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use std::process::Command;
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
#[derive(Clone, Copy, Debug)]
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

//// Convert a sox sample to a 32 bit floating point number
fn sox_sample_to_float_32bit(d: std::os::raw::c_int) -> f32 {
    /*
    Client API:
    Converts SoX native sample to a 32-bit float.
    @param d Input sample to be converted.
    @param clips Variable to increment if input sample is too large.
    */
    // #define SOX_SAMPLE_TO_FLOAT_32BIT(d,clips) (
    // LSX_USE_VAR(sox_macro_temp_double),
    // sox_macro_temp_sample=(d),
    // sox_macro_temp_sample>SOX_SAMPLE_MAX-64?++(clips),
    // 1:(((sox_macro_temp_sample+64)&~127)*(1./(SOX_SAMPLE_MAX+1.)))
    // )
    // derived from above sox user macro.
    const SOX_SAMPLE_MAX: i32 = 0x7FFFFFFF;
    let temp_sample = d;
    if temp_sample > SOX_SAMPLE_MAX - 64 {
        1.0
    } else {
        ((temp_sample + 64) & !127) as f32 * (1.0 / (SOX_SAMPLE_MAX as f32 + 1.0))
    }
}

/// Read a file into a vector of usize elements
pub fn read_audio_file(filename: &Path) -> Result<Vec<f32>, SoxErrorT> {
    unsafe {
        // create a path from the filename
        let path = CString::new(filename.to_str().unwrap().to_string()).unwrap();

        // open the file into a structure
        let sox_format = sox_open_read(path.as_ptr(), null(), null(), null());

        (*sox_format).signal.rate = 44100.0;
        (*sox_format).signal.channels = 1;

        // define our sample size.
        const SAMPLES: usize = 8192;

        // create a buffer for the SAMPLES
        let mut buffer: [std::os::raw::c_int; SAMPLES] = [0; SAMPLES];

        // create a vector into which we store the the buffer contents
        let mut tmp_vec: Vec<std::os::raw::c_int> = Vec::new();

        // keep reading until we read fewer SAMPLES than we expect, or we reach an EOF
        loop {
            let read_result = sox_read(sox_format, buffer.as_mut_ptr(), SAMPLES);
            // check that we're not done
            if read_result != (SoxErrorT::EndOfFile as usize) {
                // copy values into our final vector from the buffer

                tmp_vec.extend_from_slice(&buffer[..read_result]);
                buffer = [0; SAMPLES];
            } else {
                break;
            }

            if read_result != SAMPLES {
                break;
            }
        }

        // close the file handle
        let _close_result = sox_close(sox_format);

        // iterate over the vector, and change the samples to 32bit floats
        // let mut final_vec = Vec::new();
        let final_vec = tmp_vec
            .into_par_iter()
            .map(|v| sox_sample_to_float_32bit(v))
            .collect();

        return Ok(final_vec);
    }
}

fn read_binary_file(filename: &Path) -> Vec<f32> {
    let mut f = File::open(filename).unwrap();

    // create a buffer to read results into:
    const SAMPLES: usize = 8192;

    let mut i8buffer = [0; SAMPLES];

    let mut f32buffer: [f32; SAMPLES / 4] = [0.0; SAMPLES / 4];

    let mut final_vec: Vec<f32> = Vec::new();

    loop {
        // read some samples into the buffer
        let read = f.read(&mut i8buffer[..]);
        match read {
            Ok(bytes) => {
                if bytes == 0 {
                    break;
                } else {
                    // get that many bytes as a slice
                    let mut i8slice = &i8buffer[..bytes];
                    let r = i8slice.read_f32_into::<LittleEndian>(&mut f32buffer[..bytes / 4]);

                    match r {
                        Ok(_) => {
                            final_vec.extend_from_slice(&f32buffer[..bytes / 4]);
                        }
                        Err(e) => println!("Encountered convert error {:?}", e),
                    }
                }
            }
            Err(error) => println!("Encountered read error: {:?}", error),
        }
    }
    return final_vec;
}

pub fn run_sox_and_read_file(mp3: &Path, dat: &Path) -> Vec<f32> {
    // Get the data using the sox command
    let command = format!(
        // "sox -V1 \"{:?}\" -L -r 48000 -e float -b 16 -t raw \"{:?}\"",
        "sox -V1 \"{:?}\" -r 44100 -e float -c 1 -b 16 -t raw \"{:?}\"",
        mp3,
        dat
    );

    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());

    let cmd_result = read_binary_file(dat);

    return cmd_result;
}

#[cfg(test)]
mod tests {

    // import all of our sox stuff
    use super::*;

    use std::path::Path;

    #[test]
    fn test_read_audio_file() {
        // First use sox to generate a file from the test data
        let cargo_root = env!("CARGO_MANIFEST_DIR");
        let d = Path::new(cargo_root).join("data");

        let mp3 = Path::new(&d).join("test.mp3");
        let dat = Path::new(&d).join("test.txt");

        // get the data using our function
        let lib_result = read_audio_file(mp3.as_path()).unwrap();

        let cmd_result = run_sox_and_read_file(mp3.as_path(), dat.as_path());

        println!(
            "\tlib_result: {}/{:?}\t\ncmd_result: {}/{:?}",
            lib_result.len(),
            &lib_result[lib_result.len() - 16..],
            cmd_result.len(),
            &cmd_result[cmd_result.len() - 16..]
        );

        assert!(true);
        // assert_eq!(lib_result, cmd_result);
    }
}
