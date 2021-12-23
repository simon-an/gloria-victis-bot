use std::time::Duration;
use std::thread::sleep;

use windows::{
    Gaming::UI::*,
    Gaming::Preview::GamesEnumeration::*,
    Foundation::IAsyncOperation,
    ApplicationModel::*,
};

#[tokio::main]
async fn main() -> windows::runtime::Result<()> {

    for e in GameList::FindAllAsync()?.await?{

        println!("{} - {} - {} - {:?} ", e.TitleId()?, e.DisplayInfo()?.DisplayName()?, e.DisplayInfo()?.Description()?, e.Category()?);
    }
    // let token = GameBar::VisibilityChanged(|handler| {})?;

    loop {
        
        let gb = GameBar::Visible()?;
        let gb2 = GameBar::IsInputRedirected()?;
        println!("gamebar: {}  {}", &gb , &gb2);
        sleep(Duration::from_secs(1));
    }


    // let gco = GameChatOverlay::GetDefault()?;
    // gco.SetDesiredPosition(GameChatOverlayPosition::TopLeft)?;

    Ok(())
}