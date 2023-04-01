#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!("./voicevox_core.rs");

use std::ops::Deref;
use std::ffi::CStr;

pub type InitializeOptions = VoicevoxInitializeOptions;
pub type AudioQueryOptions = VoicevoxAudioQueryOptions;
pub type SynthesisOptions = VoicevoxSynthesisOptions;
pub type TtsOptions = VoicevoxTtsOptions;

/// Enum that represents the result of a Voicevox operation.
#[repr(i32)]
#[derive(Debug, PartialEq, Eq)]
pub enum ResultCode {
    /// Success
    Ok = 0,
    /// Failed to load Open JTalk dictionary file
    NotLoadedOpenjtalkDictError = 1,
    /// Failed to load the model
    LoadModelError = 2,
    /// Failed to get supported device information
    GetSupportedDevicesError = 3,
    /// GPU mode is not supported
    GpuSupportError = 4,
    /// Failed to load meta information
    LoadMetasError = 5,
    /// Status is uninitialized
    UninitializedStatusError = 6,
    /// Invalid speaker ID specified
    InvalidSpeakerIdError = 7,
    /// Invalid model index specified
    InvalidModelIndexError = 8,
    /// Inference failed
    InferenceError = 9,
    /// Failed to output context labels
    ExtractFullContextLabelError = 10,
    /// Invalid UTF-8 string input
    InvalidUtf8InputError = 11,
    /// Failed to parse Aquestalk-style text
    ParseKanaError = 12,
    /// Invalid AudioQuery
    InvalidAudioQueryError = 13,
}

///
/// Hardware acceleration mode
///
#[derive(Copy, Clone)]
pub enum AccelerationMode {
    Auto = VoicevoxAccelerationMode_VOICEVOX_ACCELERATION_MODE_AUTO as isize,
    CPU = VoicevoxAccelerationMode_VOICEVOX_ACCELERATION_MODE_CPU as isize,
    GPU = VoicevoxAccelerationMode_VOICEVOX_ACCELERATION_MODE_GPU as isize,
}

///
/// Provides functionality of Voicevox Core.
///
pub struct VoicevoxCore;

impl VoicevoxCore {
    pub fn new(opt: InitializeOptions) -> Result<Self, ResultCode> {
        let result = unsafe {
            voicevox_initialize(opt)
        };

        match result {
            0 => Ok(Self {}),
            e => Err( unsafe { std::mem::transmute(e) } ),
        }
    }

    pub fn new_from_options(acceleration_mode: AccelerationMode,
                cpu_num_threads: u16,
                load_all_models: bool,
                open_jtalk_dict_dir: &std::ffi::CStr
               ) -> Result<Self, ResultCode> {

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

    pub fn get_version() -> &'static str {
        let version_ptr = unsafe {
            voicevox_get_version()
        };

        let version_cstr = unsafe {
            std::ffi::CStr::from_ptr(version_ptr)
        };

        version_cstr.to_str().unwrap()
    }

    ///
    /// Returns the metadata of all the available voice models in JSON format.
    ///
    /// # Returns
    ///
    /// Returns a string representing the metadata of all available voice models in JSON format.
    ///
    pub fn get_metas_json() -> &'static str {
        unsafe { CStr::from_ptr(voicevox_get_metas_json()).to_str().unwrap() }
    }

    ///
    /// Returns the list of devices supported by Voicevox in JSON format.
    ///
    /// # Returns
    ///
    /// Returns a string representing the list of devices supported by Voicevox in JSON format.
    ///
    pub fn get_supported_devices_json() -> &'static str {
        unsafe { CStr::from_ptr(voicevox_get_supported_devices_json()).to_str().unwrap() }
    }

    ///
    /// Loads a model for the specified speaker ID.
    ///
    /// # Arguments
    ///
    /// * `speaker_id` - The ID of the speaker to load the model for.
    ///
    /// # Returns
    ///
    /// If the model was loaded successfully, returns `Ok(())`.
    /// If an error occurred, returns a `ResultCode` enum value representing the error.
    ///
    pub fn load_model(&self, speaker_id: u32) -> Result<(), ResultCode> {
        let result_code = unsafe { voicevox_load_model(speaker_id) };
        if result_code == ResultCode::Ok as i32 {
            Ok(())
        } else {
            Err( unsafe { std::mem::transmute(result_code) } )
        }
    }

    ///
    /// Returns a boolean value indicating whether the current process is running in GPU mode.
    ///
    /// # Returns
    ///
    /// Returns `true` if the current process is running in GPU mode, and `false` otherwise.
    ///
    pub fn is_gpu_mode(&self) -> bool {
        unsafe { voicevox_is_gpu_mode() }
    }

    ///
    /// Returns a boolean value indicating whether a voice model with the specified speaker ID has been loaded.
    ///
    /// # Arguments
    ///
    /// * `speaker_id` - The ID of the speaker to check for.
    ///
    /// # Returns
    ///
    /// Returns `true` if a voice model with the specified speaker ID has been loaded, and `false` otherwise.
    ///
    pub fn is_model_loaded(&self, speaker_id: u32) -> bool {
        unsafe { voicevox_is_model_loaded(speaker_id) }
    }

    /// This function generates a WAV file with the result of text-to-speech synthesis using Voicevox Core.
    /// This is simple version of [VoicevoxCore::tts].
    ///
    /// # Arguments
    ///
    /// * `text` - The text to be synthesized.
    /// * `speaker_id` - The ID of the speaker to be used for the synthesis.
    ///
    /// # Returns
    ///
    /// * [Wav] if the synthesis succeeds.
    /// * An error code otherwise.
    pub fn tts_simple(&self,
                text: &str,
                speaker_id: u32) -> Result<Wav, ResultCode> {
        Self::_tts(text, speaker_id, Self::make_default_tts_options())
    }

    /// This function generates a WAV file with the result of text-to-speech synthesis using Voicevox Core.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to be synthesized.
    /// * `speaker_id` - The ID of the speaker to be used for the synthesis.
    /// * `options` - The options for the synthesis.
    ///
    /// # Returns
    ///
    /// * [Wav] if the synthesis succeeds.
    /// * An error code otherwise.
    pub fn tts(&self,
                text: &str,
                speaker_id: u32,
                options: TtsOptions) -> Result<Wav, ResultCode> {
        Self::_tts(text, speaker_id, options)
    }

    fn _tts(text: &str,
            speaker_id: u32,
            options: TtsOptions) -> Result<Wav, ResultCode> {
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

            e => Err( unsafe { std::mem::transmute(e) } ),
        }
    }

    /// Converts a ResultCode to an error message.
    ///
    /// # Arguments
    ///
    /// * result_code - A ResultCode to convert.
    ///
    pub fn error_result_to_message(result_code: ResultCode) -> &'static str {
        unsafe {
            let message = voicevox_error_result_to_message(result_code as i32);
            CStr::from_ptr(message).to_str().unwrap()
        }
    }

}


///
/// Holds the WAV file generated by the Voicevox core and performs memory management through RAII.
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

