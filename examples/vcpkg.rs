// build.rs
fn main() {
    vcpkg::find_package("leptonica").unwrap();
}
