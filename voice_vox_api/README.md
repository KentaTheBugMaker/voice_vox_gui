# VoiceVox openapi binding for rust.
 VoiceVox is a japanese deep learning based Text to Speech software with many customizable parameters.

## How to use
 1. install VoiceVox
 2. add to dependency with voice_vox_api and tokio.
 3. startup VoiceVox or VoiceVox Engine eg. run.exe
 4. fill parameters in api fields and .call().await.
## use with async-std / wasm
```toml
voice_vox_api={version="0.13.4",features = ["backend_surf"]}
```
## use with tokio / async-std+tokio02 feature / wasm
```toml
voice_vox_api={version="0.13.4",features = ["backend_reqwest"]}
```

## works grate crates.
 * egui / iced - gui crates
 * rodio - audio playback.
## sample
 * [voice_vox](https://github.com/t18b219k/voice_vox_gui/tree/master/voice_vox)
## versioning
 * use same version number with VoiceVox engine.
 * some fix applied without announce
 * maybe break semver.
## fix
 * fix pause mora in project file.  
 * use f64 instead of f32 to provicde same precision as VoiceVox.