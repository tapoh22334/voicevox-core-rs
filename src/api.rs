#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!("./voicevox_core.rs");

use std::ops::Deref;

pub type InitializeOptions = VoicevoxInitializeOptions;
pub type AudioQueryOptions = VoicevoxAudioQueryOptions;
pub type SynthesisOptions = VoicevoxSynthesisOptions;
pub type TtsOptions = VoicevoxTtsOptions;

///
/// ハードウェアアクセラレーションモード
///
#[derive(Copy, Clone)]
pub enum AccelerationMode {
    Auto = VoicevoxAccelerationMode_VOICEVOX_ACCELERATION_MODE_AUTO as isize,
    CPU = VoicevoxAccelerationMode_VOICEVOX_ACCELERATION_MODE_CPU as isize,
    GPU = VoicevoxAccelerationMode_VOICEVOX_ACCELERATION_MODE_GPU as isize,
}

///
/// Voicevox Core の機能を提供する
///
pub struct VoicevoxCore;

impl VoicevoxCore {
    pub fn new(opt: InitializeOptions) -> Result<Self, VoicevoxResultCode> {
        let result = unsafe {
            voicevox_initialize(opt)
        };

        match result {
            0 => Ok(Self {}),
            e => Err(e),
        }
    }

    pub fn new_from_options(acceleration_mode: AccelerationMode,
                cpu_num_threads: u16,
                load_all_models: bool,
                open_jtalk_dict_dir: &std::ffi::CStr
               ) -> Result<Self, VoicevoxResultCode> {

        let opt = InitializeOptions {
            acceleration_mode: acceleration_mode as i32,
            cpu_num_threads,
            load_all_models,
            open_jtalk_dict_dir: open_jtalk_dict_dir.as_ptr()
        };

        Self::new(opt)
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
    pub fn get_version() -> &'static str {
        let version_ptr = unsafe {
            voicevox_get_version()
        };

        let version_cstr = unsafe {
            std::ffi::CStr::from_ptr(version_ptr)
        };

        version_cstr.to_str().unwrap()
    }

    pub fn make_default_initialize_options() -> InitializeOptions {
        let opt = unsafe {
            voicevox_make_default_initialize_options()
        };

        opt
    }

    pub fn make_default_tts_options() -> TtsOptions {
        unsafe {
            voicevox_make_default_tts_options()
        }
    }

    fn _tts(text: &str,
            speaker_id: u32,
            options: TtsOptions) -> Result<Wav, VoicevoxResultCode> {
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

    pub fn tts_simple(&self,
                text: &str,
                speaker_id: u32) -> Result<Wav, VoicevoxResultCode> {
        Self::_tts(text, speaker_id, Self::make_default_tts_options())
    }

    pub fn tts(&self,
                text: &str,
                speaker_id: u32,
                options: TtsOptions) -> Result<Wav, VoicevoxResultCode> {
        Self::_tts(text, speaker_id, options)
    }

}


///
/// Voicevox Coreによって生成されるwavファイルを保持する
/// RAIIによるメモリ管理を行う
///
pub struct Wav {
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

