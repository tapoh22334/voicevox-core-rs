use voicevox_core_rs::*;
use std::io::Write;

fn main() -> std::io::Result<()> {
    println!("VOICEVOX CORE version: {}", VoicevoxCore::get_version());

    let dir = std::ffi::CString::new("open_jtalk_dic_utf_8-1.11").unwrap();
    let vvc = match VoicevoxCore::new_from_options(AccelerationMode::Auto,
                                                        0,
                                                        true,
                                                        dir.as_c_str()) {
        Ok(vvc) => vvc,
        Err(e) => panic!("failed to initialize voicevox {:?}", e),
    };

    {
        let text: &str = "こんにちは";
        let speaker: u32 = 1;

        let wav = match vvc.tts_simple(text, speaker) {
            Ok(wav) => wav,
            Err(e) => panic!("failed to generate wav {:?}", e),
        };

        let mut file = std::fs::File::create("audio.wav").unwrap();
        file.write_all(&wav.as_slice())?;
    }

    Ok(())
}
