//! FFI Bindings for VOICEVOX CORE
//!
//! # Example
//!
//! ```
//! use std::io::Write;
//! use vvcore::*;
//!
//! let dir = std::ffi::CString::new("./voicevox_core/open_jtalk_dic_utf_8-1.11").unwrap();
//! let vvc = VoicevoxCore::new_from_options(AccelerationMode::Auto, 0, true, dir.as_c_str()).unwrap();
//!
//! let text: &str = "こんにちは";
//! let speaker: u32 = 1;
//! let wav = vvc.tts_simple(text, speaker).unwrap();
//!
//! let mut file = std::fs::File::create("audio.wav").unwrap();
//! file.write_all(&wav.as_slice()).unwrap();
//! ```

pub mod api;

pub use self::api::{
    VoicevoxCore,
    ResultCode,
    AccelerationMode,
    CPointerWrap,
    CStrWrap,
    InitializeOptions,
    AudioQueryOptions,
    SynthesisOptions,
    TtsOptions,
};
