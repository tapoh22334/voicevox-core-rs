use vvc;
use std::io::Write;

fn main() -> std::io::Result<()> {
    println!("VOICEVOX CORE version: {}", vvc::VoicevoxCore::get_version());

    //let mut opt = vvc::VoicevoxCore::make_default_initialize_options();
    let dir = std::ffi::CString::new("open_jtalk_dic_utf_8-1.11").unwrap();

    //let vvc = match vvc::VoicevoxCore::new(opt) {
    let vvc = match vvc::VoicevoxCore::new_from_options(vvc::AccelerationMode::Auto,
                                                        0,
                                                        true,
                                                        dir.as_c_str()) {
        Ok(vvc) => vvc,
        Err(e) => panic!("failed to initialize voicevox {}", e),
    };

    {
        let text: &str = "こんにちは";
        let speaker: u32 = 1;

        let wav = match vvc.tts_simple(text, speaker) {
            Ok(wav) => wav,
            Err(e) => panic!("failed to generate wav {}", e),
        };

        let mut file = std::fs::File::create("audio.wav").unwrap();
        file.write_all(&*wav)?;
    }

    Ok(())
}
