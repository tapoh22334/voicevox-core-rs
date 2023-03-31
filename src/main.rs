#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!("./voicevox_core.rs");

use std::ops::Deref;
use std::io::Write;

struct VoicevoxCore;

impl VoicevoxCore {
    pub fn new(opt: VoicevoxInitializeOptions) -> Result<Self, VoicevoxResultCode> {
        let result = unsafe {
            voicevox_initialize(opt)
        };

        match result {
            0 => Ok(Self {}),
            e => Err(e),
        }
    }
}

impl Drop for VoicevoxCore {
    fn drop(&mut self) {
        unsafe {
            voicevox_finalize()
        };

    }
}

impl VoicevoxCore {
    pub fn make_default_initialize_options() -> VoicevoxInitializeOptions {
        let opt = unsafe {
            voicevox_make_default_initialize_options()
        };

        opt
    }

    pub fn make_default_tts_options() -> VoicevoxTtsOptions {
        unsafe {
            voicevox_make_default_tts_options()
        }
    }

    pub fn get_version() -> &'static str {
        let version_ptr = unsafe {
            voicevox_get_version()
        };

        let version_cstr = unsafe {
            std::ffi::CStr::from_ptr(version_ptr)
        };

        version_cstr.to_str().unwrap()
    }

    pub fn tts(&self,
                text: &str,
                speaker_id: u32,
                options: VoicevoxTtsOptions) -> Result<Wav, VoicevoxResultCode> {

        let c_str = std::ffi::CString::new(text).unwrap();
        let mut out_length: usize = 0;
        let mut out_wav: *mut u8 = std::ptr::null_mut();

        let result = unsafe {
            voicevox_tts(c_str.as_ptr(),
                        speaker_id,
                        options,
                        &mut out_length,
                        &mut out_wav
                        )
        };

        match result {
            0 => {
                let wav = Wav::new(out_wav, out_length);
                Ok(wav)
            }

            e => Err(e),
        }

    }
}

struct Wav {
    bytes: *mut u8,
    length: usize
}

impl Wav {
    pub fn new(bytes: *mut u8, length: usize) -> Self {
        Self {bytes, length}
    }
}

impl Deref for Wav {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.bytes, self.length) }
    }
}

impl Drop for Wav {
    fn drop(&mut self) {
        unsafe {
            voicevox_wav_free(self.bytes);
        };
    }
}


