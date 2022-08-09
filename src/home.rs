use macroquad::prelude::*;


pub fn display_home(em: f32) {

     draw_text(
        "Press enter to start...",
            screen_width() - 12.0*(5.0 + em*0.7),
            21.0 * em,
            5.0 + em*0.7,
            DARKGREEN,
        );

}