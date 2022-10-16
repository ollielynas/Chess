use macroquad::prelude::*;

use crate::{game_data::GameData, home::{Screen, UserData}};




pub fn draw_death_screen(data: &mut GameData, user: &mut UserData, em: f32) {
    if is_key_pressed(KeyCode::Escape) {
        data.screen = Screen::Home;
    }
    draw_text(
        "You Died",
        6.0*em,
        6.0*em,
        em * 3.0,
        RED
    );

    draw_text(&format!("Round: {}{:?}| {}", data.round, (0..15-data.round.to_string().len()).map(|_|" ").collect::<String>(), user.high_round).replace("\"", ""), em * 5.0, 8.0 * em, em, GRAY);
    draw_text(&format!("Score: {}{:?}| {}", (data.score*100.0).round()/100.0, (0..15-data.score.to_string().len()).map(|_|" ").collect::<String>(), (user.high_score*100.0).round() / 100.0).replace("\"", ""), em * 5.0, 9.5 * em, em, GRAY);
}