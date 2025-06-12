mod app;

// use egui::{FontData, FontDefinitions, FontFamily};

impl eframe::App for app::AliasApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.ui(ctx);
    }
}


// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| {
                    // note: todo: i want to fetch the font from the server and then add it here.
                    // add_fonts_to_ctx(&cc.egui_ctx);
                    Ok(Box::new(app::AliasApp::default()))
                }),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}

// pub fn add_fonts_to_ctx(egui_ctx: &egui::Context) {
//     use std::{collections::BTreeMap, sync::Arc};

//     let mut font_data: BTreeMap<String, Arc<FontData>> = BTreeMap::new();

//     let mut families = BTreeMap::new();

//     font_data.insert(
//         "Hack".to_owned(),
//         Arc::new(FontData::from_static(crate::fonts::HACK)),
//     );

//     families.insert(
//         FontFamily::Monospace,
//         vec![
//             "Hack".to_owned(),
//         ],
//     );
//     families.insert(
//         FontFamily::Proportional,
//         vec![
//             "Hack".to_owned(),
//         ],
//     );

//     let fd = FontDefinitions {
//         font_data,
//         families,
//     };

//     egui_ctx.set_fonts(fd);
// }
