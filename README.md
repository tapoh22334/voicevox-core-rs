## VOICEVOX CORE Rust Bindings

VOICEVOX COREのRust向けFFIラッパーです。

## サンプルの実行方法

### 必要なもの

以下の方法でVOICEVOX COREをダウンロードしておいてください。
https://github.com/VOICEVOX/voicevox_core#%E7%92%B0%E5%A2%83%E6%A7%8B%E7%AF%89

### ビルド方法

以下のコマンドでサンプルコードをビルドします。
```
cargo build --example simple-tts
```

ビルドに必要なファイルがあるディレクトリに実行ファイルを移動します。
```
mv target/debug/examples/simple-tts voicevox_core
```

### 実行

以下のコマンドで実行します。voicevox_coreディレクトリ内にaudio.wavが生成されます。
```
(export LD_LIBRARY_PATH=.:$LD_LIBRARY_PATH && cd voicevox_core && ./simple-tts)
```


## 対応状況

以下の関数は、高レベルなＡＰＩとして利用可能です。
また、すべての関数はapiモジュール内でunsafeな関数として参照できます。
初期化関数や、free関数はRAIIにより実行されるため実装されません。

 - [x] voicevox_make_default_initialize_options
 - [x] ~~voicevox_initialize~~
 - [x] voicevox_get_version
 - [x] voicevox_load_model
 - [x] voicevox_is_gpu_mode
 - [x] voicevox_is_model_loaded
 - [x] ~~voicevox_finalize~~
 - [x] voicevox_get_metas_json
 - [x] voicevox_get_supported_devices_json
 - [ ] voicevox_predict_duration
 - [ ] ~~voicevox_predict_duration_data_free~~
 - [ ] voicevox_predict_intonation
 - [ ] ~~voicevox_predict_intonation_data_free~~
 - [ ] voicevox_decode
 - [ ] ~~voicevox_decode_data_free~~
 - [ ] voicevox_make_default_audio_query_options
 - [ ] voicevox_audio_query
 - [ ] voicevox_make_default_synthesis_options
 - [ ] voicevox_synthesis
 - [x] voicevox_make_default_tts_options
 - [x] voicevox_tts
 - [ ] ~~voicevox_audio_query_json_free~~
 - [x] ~~voicevox_wav_free~~
 - [x] voicevox_error_result_to_message
