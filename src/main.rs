
mod game;
mod craster;
mod cwin;
mod cren;
mod cmath;

use game::Game;
use cren::CRen;
use cwin::CWin;

fn main() {

    let mut ren: CRen = CRen::new(1066, 800);
    let mut win: cwin::CWin = CWin::new(&ren.raster);

    let mut c: f64 = 0.0;
    let mut s: f64 = 0.0;
    let r: f64 = 100.0;
    let mut theta: f64 = 0.0;

    while win.is_open() {

        // ren.fill(0x00ff00);
        theta += 0.01;
        if theta > 2.0*3.142 { theta -= 2.0*3.142; }
        
        c = cmath::cos(theta);
        s = cmath::sin(theta);

        ren.draw_line(300,300, (300.0 + r*c) as usize, (300.0 + r*s) as usize, 0xffffff);

        win.draw(&ren.raster);
        ren.clear();

    };

}
