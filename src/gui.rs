
// use imgui::*;
// use inputbot::MouseCursor;


// use crate::Resolution;
// use crate::desktop::{pos, window_size};
// use crate::{game_interactions, shared::BotMode, statics};


// pub fn create() {
//     let mut state = State::default();
//     let system = crate::support::init(file!());
//     system.main_loop(move |_, ui| {
//         Window::new("GV BOT")
//             .size([400.0, 250.0], Condition::FirstUseEver)
//             .position([0.0, 0.0], Condition::Always)
//             .bg_alpha(255f32)
//             .build(ui, || {
//                 ui.text(format!("Running: ({:?})", statics::RUNNING.lock().unwrap(),));
//                 ui.separator();
//                 ui.text(format!("Counter: ({:?})", statics::COUNTER.lock().unwrap(),));
//                 ui.separator();
//                 let (x, y) = MouseCursor::pos();
//                 ui.text(format!("Mouse Position: ({:},{:})", x, y));
//                 ui.separator();
              
//                 let (rect, w, h) = window_size();
//                 ui.text(format!("window_size: ({:},{:})", w, h));
//                 ui.text(format!("pos: ({:},{:})", rect.left, rect.top));
//                 let window_pos = (rect.left, rect.top);
//                 let (x, y) = pos();
//                 ui.separator();
//                 ui.text(format!("Mouse Position: ({:},{:})", x-window_pos.0, y-window_pos.1));
//                 ui.separator();
//                 let resolution = match (w, h) {
//                     (5120, 1440) => Resolution::TWICE_QHD,
//                     (1936, 1119) => Resolution::FULL_HD,
//                     (1040, 807) => Resolution::SMALLEST,
//                     _ => Resolution::ANY,
//                 };
//                 ui.separator();
//                 ui.text(format!("Detected Resolution: {:?}", resolution));

//                 let value = BotMode::values();

//                 for v in value {
//                     let label = v.to_string();
//                     let ex1 = ui.radio_button(<String as AsRef<str>>::as_ref(&label), &mut state.mode, v);
//                     if ex1 {
//                         game_interactions::bind_keys_for(v, window_pos, rect, resolution);
//                     }
//                 }
//             });
//     });
// }
// #[derive(Default)]
// struct State {
//     mode: BotMode,
// }