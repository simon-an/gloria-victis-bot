use windows::UI::Input::*;
use inputbot::MouseCursor;

fn main() -> windows::runtime::Result<()> {
    loop {
        let (x, y) = MouseCursor::pos();
        println!("{}/{}", x, y);
    }
    Ok(())
}
