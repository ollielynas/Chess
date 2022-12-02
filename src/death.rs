use macroquad::prelude::*;
use savefile::save_file;

use crate::{
    game_data::GameData,
    home::{Screen, UserData, new_game},
    GLOBAL_VERSION,
};

pub fn draw_death_screen(data: &mut GameData, user: &mut UserData, em: f32) {
    if is_key_pressed(KeyCode::Escape) {
        data.screen = Screen::Home;
    }
    draw_text("You Died", 6.0 * em, 6.0 * em, em * 3.0, RED);

    let mouse_x = mouse_position().0 / em;
    let mouse_y = mouse_position().1 / em;

    draw_text("New Game", 3.0 * em, screen_height() - 3.0*em, em, WHITE);
    draw_text(
        "Home",
        15.0 * em,
        screen_height() - 3.0*em,
        em,
        WHITE,
    );
    if is_mouse_button_pressed(MouseButton::Left) {
        if mouse_x > 3.0 && mouse_x < 6.0 && mouse_y > screen_height()/em - 4.0 && mouse_y < screen_height()/em - 3.0 {
            data.sounds.push(("click".to_owned(), 0.0));
            data.screen = Screen::AbilitiesScreen;
        }
        if mouse_x > 15.0 && mouse_x < 18.3 && mouse_y > screen_height()/em - 4.0 && mouse_y < screen_height()/em - 3.0 {
            data.sounds.push(("click".to_owned(), 0.0));
            save_file("game_data.bin", GLOBAL_VERSION, data).unwrap();
            data.screen = Screen::Home;
        }
    }

    draw_text(
        &format!(
            "Round: {}{:?}| {}",
            data.round,
            (0..15 - data.round.to_string().len())
                .map(|_| " ")
                .collect::<String>(),
            user.high_round
        )
        .replace("\"", ""),
        em * 5.0,
        8.0 * em,
        em,
        GRAY,
    );
    draw_text(
        &format!(
            "Score: {}{:?}| {}",
            (data.score * 100.0).round() / 100.0,
            (0..15 - data.score.to_string().len())
                .map(|_| " ")
                .collect::<String>(),
            (user.high_score * 100.0).round() / 100.0
        )
        .replace("\"", ""),
        em * 5.0,
        9.5 * em,
        em,
        GRAY,
    );
}
