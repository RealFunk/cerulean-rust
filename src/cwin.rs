

use minifb::Key as MFBKey;
use minifb::WindowOptions as MFBWindowOptions;
use minifb::Window as MFBWindow;

use crate::craster;
use crate::craster::CRaster;

pub struct CWin {
    pub width: usize,
    pub height: usize,
    pub visible: bool,
    lib_window: MFBWindow,
}

impl CWin {

    pub fn new(raster: &CRaster) -> CWin {
        let mut win = MFBWindow::new(
            "Cerluean Engine",
            raster.width,
            raster.height,
            MFBWindowOptions {
                resize: false,
                ..MFBWindowOptions::default()
            },
        )
        .expect("Unable to create the window");
    
        win.set_target_fps(60);
    
        return CWin { 
            width: raster.width, 
            height: raster.height,
            visible: true,
            lib_window: win,
        };
    }

    pub fn draw(&mut self, raster: &CRaster) {
        self.lib_window
            .update_with_buffer(&raster.data, self.width, self.height)
            .unwrap();
    }

    pub fn set_visibility(&mut self, visibility: bool) {
        self.visible = visibility;
    }

    pub fn is_open(&self) -> bool {
        return self.lib_window.is_open();
    }

}

// pub fn new(w: u64, h: u64) -> CWin {

//     let mut win = MFBWindow::new(
//         "Cerluean Engine",
//         w as usize,
//         h as usize,
//         MFBWindowOptions {
//             resize: false,
//             ..MFBWindowOptions::default()
//         },
//     )
//     .expect("Unable to create the window");

//     win.set_target_fps(60);

//     return CWin { 
//         width: w, 
//         height: h,
//         visible: true,
//         lib_window: win,
//     };
// }
