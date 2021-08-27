use imgui::*;

use crate::{game_interactions, shared::BotMode, statics};

pub fn create() {
    let mut state = State::default();
    let system = crate::support::init(file!());
    system.main_loop(move |_, ui| {
        Window::new(im_str!("Hello world"))
            .size([300.0, 110.0], Condition::FirstUseEver)
            .bg_alpha(255f32)
            .build(ui, || {
               
                ui.text(format!(
                    "Running: ({:?})",
                    statics::RUNNING.lock(),
                ));
                ui.separator();
                ui.text(format!(
                    "Counter: ({:?})",
                    statics::COUNTER.lock(),
                ));
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));

                let value = BotMode::values();

                for v in value {
                    let label: ImString = v.to_string().into();
                    let ex1 = ui.radio_button(label.as_ref(), &mut state.mode, v);
                    if ex1 {
                        game_interactions::bind_keys_for(v);
                    }
                }

            });
    });
}
#[derive(Default)]
struct State {
    mode: BotMode,
}