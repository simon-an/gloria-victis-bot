use tesseract::*;

fn main() -> anyhow::Result<()> {
    let folder = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("{}", folder);
    std::env::set_var("TESSDATA_PREFIX", format!("{}\\{}", folder, "tessdata"));

    let temp = Tesseract::new_with_oem(None, Some("deu_frak"), OcrEngineMode::Default)?;
    let only_lstm_str = temp
        .set_image("images/rechts.png")?
        // .set_image("images/hoch.png")?
        .recognize()?
        .get_text()?;
    println!("{}", only_lstm_str);
    Ok(())
}
