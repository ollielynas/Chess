use std::fmt::format;

use crate::game_data::GameData;
use macroquad::prelude::*;

pub fn draw_icons(data: &GameData, em: f32) {
    draw_icon(
        screen_width() - 0.7*em, 20.65*em, 
        "Hover over a blue help icon if you are not sure what to do. To toggle help icons press this help icon".to_owned(),
        em
    );
    draw_icon(
        0.7*em, 2.65*em, 
        "Hover over a help icon if you are not sure what to do. To toggle help icons press this help icon".to_owned(),
        em
    )
}

fn draw_icon(x:f32, y:f32, txt: String, em: f32) {
    draw_circle (
        x, y, em*0.1,Color::from_rgba(
            150,150,250,200
        )
    );

    if (mouse_position().0 - x).hypot(mouse_position().1 - y) < 0.1 * em {

        let mut text = vec![txt.chars().collect::<Vec<char>>()];
        let max_length = 50;
        for i in 0..30 {
            text.push(vec![]);
            while text[i].len() > max_length || &text[i][&text[i].len()-1] != &' ' {
                let item = text[i].pop().expect("text error");
                text[i+1].insert(0,item);
            }

            let mut valid = true;
            for i in &text {
                if i.len() > max_length {
                    valid = false;
                }
            }
            if valid {break}
        }

        let offset_x  = match x {
            x if x > screen_width()/2.0 => em*-13.5,
            _ => 0.0
        };
        let offset_y  = match y {
            y if y > screen_height()/2.0 => em*1.0 + em * text.len() as f32 * 0.7,
            _ => 0.0
        };
        
        draw_rectangle (
            x,y, 
            em*13.5 + offset_x, 
            em*1.0 + em * text.len() as f32 * 0.7 + offset_y,
            WHITE
        );
        
        for i in 0..text.len() {
            draw_text(
            &String::from_iter(text[i].clone()),
            x + em*0.5 + offset_x, 
            y+i as f32 *em*0.8 + em + offset_y, 
            em*0.6, BLACK)
        }

        draw_circle (
            x, y, em*0.1,Color::from_rgba(
                250,250,250,200
            )
        );


    }
}