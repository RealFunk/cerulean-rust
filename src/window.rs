
pub struct Window {
    pub width: u64,
    pub height: u64,
    pub pixels: Vec<u32>,
}

pub impl Window {

    pub fn new_window(w: u64, h: u64) -> Window {
        return Window { 
            width: w, 
            height: h, 
            pixels: vec![0; ((w*h) as usize).try_into().unwrap()],
        }
    }

}

/*
pub strict Window {
    pub (width, height): (u64, u64),
    pub pixels: Vec<u32>,
    pub title: &str,
    pub is_open: bool,
    // pub default_close_operation: ???,
}
*/

pub fn new(w: u64, h: u64) -> Window {
    return Window { 
        width: w, 
        height: h, 
        pixels: vec![0; ((w*h) as usize).try_into().unwrap()],
    }
}

