// build.rs

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    if let Some(out_dir) = env::var_os("USERPROFILE") {
        let dest_path = Path::new(&out_dir).join(".cargo/registry/src/github.com-1ecc6299db9ec823/opencv-0.53.2/src_cpp/ocvrs_ephemeral.hpp");
        fs::write(
            &dest_path,
            ""
        ).unwrap();
    } else {
        // WARN
    }
}

