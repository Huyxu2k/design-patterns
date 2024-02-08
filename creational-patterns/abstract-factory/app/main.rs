mod render;

use render::render;
use macos_gui::factory::MacFactory;
use windows_gui::factory::WindowsFactory;


fn main(){
    let window=true;

    if window{
        render(WindowsFactory);
    }else {
        render(MacFactory)
    }

}