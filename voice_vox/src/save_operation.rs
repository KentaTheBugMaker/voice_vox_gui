#[cfg(target_arch = "wasm32")]
pub(crate) fn save_as(history: &mut HistoryManager, opening_file: &mut Option<String>) {
    use wasm_bindgen::JsCast;

    use crate::history;
    let window = web_sys::window().expect("No window");
    let document = window.document().expect("No document");

    let link: web_sys::HtmlAnchorElement = document
        .create_element("a")
        .expect("failed to create HtmlAnchorElement")
        .dyn_into()
        .expect("failed to cast HtmlAnchorElement");
    //
    let json = serde_json::to_string(&history.project).expect("failed to build vvproj");

    let blob = web_sys::Blob::new_with_str_sequence(&json.into()).expect("failed to build blob");
    let url = web_sys::Url::create_object_url_with_blob(&blob)
        .expect("failed to build object url from blob");
    link.set_href(&url);
    link.click();
    history.save();
}

use crate::history::HistoryManager;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn save_as(history: &mut HistoryManager, opening_file: &mut Option<String>) {
    let file = rfd::FileDialog::new()
        .add_filter("VoiceVox project file", &["vvproj"])
        .set_directory("/")
        .save_file();
    if let Some(path) = file {
        std::fs::write(
            path.clone(),
            serde_json::to_string(&history.project).unwrap(),
        )
        .unwrap();
        let path = path.to_str().map(|st| st.to_owned());

        *opening_file = path;
        history.save();
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn save(history: &mut HistoryManager, opening_file: &Option<String>) {
    if let Some(path) = opening_file {
        std::fs::write(
            path.clone(),
            serde_json::to_string(&history.project).unwrap(),
        )
        .unwrap();

        history.save();
    }
}
#[cfg(target_arch = "wasm32")]
pub(crate) fn load(history: &mut HistoryManager, opening_file: &mut Option<String>) {
    let file = rfd::AsyncFileDialog::new()
        .add_filter("VoiceVox project file", &["vvproj"])
        .pick_file();
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) async fn load(history: &mut HistoryManager, opening_file: &mut Option<String>) {
    let file = rfd::AsyncFileDialog::new()
        .add_filter("VoiceVox project file", &["vvproj"])
        .set_directory("/")
        .pick_file();
    if let Some(file_handle) = file.await {
        let path = file_handle.path();
        if let Ok(json) = std::fs::read_to_string(file_handle.path()) {
            let vvproj = serde_json::from_str(&json).unwrap();
            *opening_file = path.to_str().map(|st| st.to_owned());
            *history = crate::history::HistoryManager::from_project(vvproj);
        }
    }
}
