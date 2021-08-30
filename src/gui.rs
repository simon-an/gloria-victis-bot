use imgui::*;

use crate::{game_interactions, shared::BotMode, statics};

pub fn create() {
    let mut state = State::default();
    let system = crate::support::init(file!());
    system.main_loop(move |_, ui| {
        Window::new(im_str!("GV BOT"))
            .size([400.0, 250.0], Condition::FirstUseEver)
            .position([0.0, 0.0], Condition::Always)
            .bg_alpha(255f32)
            .build(ui, || {
                ui.text(format!("Running: ({:?})", statics::RUNNING.lock().unwrap(),));
                ui.separator();
                ui.text(format!("Counter: ({:?})", statics::COUNTER.lock().unwrap(),));
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
                ui.separator();

                let value = BotMode::values();

                for v in value {
                    let label: ImString = v.to_string().into();
                    let ex1 = ui.radio_button(label.as_ref(), &mut state.mode, v);
                    if ex1 {
                        game_interactions::bind_keys_for(v,  (mouse_pos[0] as i32, mouse_pos[1] as i32));
                    }
                }
            });
    });
}
#[derive(Default)]
struct State {
    mode: BotMode,
}
