use macroquad::prelude::*;


pub fn display_home(em: f32) {

    draw_text(
        "Press enter to start...",
            screen_width() - 12.0*(5.0 + em*0.7),
            21.0 * em,
            5.0 + em*0.7,
            DARKGREEN,
        );
        
    draw_text(
        "One True King",
            1.0*em,
            3.0*em,
            8.0 + em*1.5,
            DARKGREEN,
        );
    draw_text("abilities",1.0*em,5.0*em,5.0 + em,GRAY);
    draw_text(&format!("1.) {}", 1),1.0*em,6.0*em,4.0 + em,GRAY);
    draw_text(&format!("2.) {}", 1),1.0*em,7.0*em,4.0 + em,GRAY);
    draw_text(&format!("3.) {}", 1),1.0*em,8.0*em,4.0 + em,GRAY);
    draw_text(&format!("4.) {}", 1),1.0*em,9.0*em,4.0 + em,GRAY);
    draw_text(&format!("5.) {}", 1),1.0*em,10.0*em,4.0 + em,GRAY);

    draw_text("Pick New",screen_width()/2.0,6.0*em,4.0 + em,ORANGE);
    draw_text("Pick New",screen_width()/2.0,7.0*em,4.0 + em,ORANGE);
    draw_text("Pick New",screen_width()/2.0,8.0*em,4.0 + em,ORANGE);
    draw_text("Pick New",screen_width()/2.0,9.0*em,4.0 + em,ORANGE);
    draw_text("Pick New",screen_width()/2.0,10.0*em,4.0 + em, ORANGE);


}