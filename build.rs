fn main(){
    println!("cargo:rustc-link-search=native=voicevox_core");
    println!("cargo:rustc-link-lib=voicevox_core");
    //println!("cargo:rustc-link-lib=onnxruntime");
    //println!("cargo:rustc-link-lib=onnxruntime_providers_shared");
}
