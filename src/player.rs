use macroquad::{prelude::*};
use crate::home::*;

pub fn player_movement(player: &mut Player, user: &UserData) -> bool {
        let mut new_move = false;

        if is_key_pressed(user.left) {
            new_move = true;
            if player.target_x > 0.0 {
                player.target_x -= 1.0
            }
        }
        else if is_key_pressed(user.right) {
            new_move = true;
            if player.target_x < 15.0 {
                player.target_x += 1.0
            }
        }
        else if is_key_pressed(user.up) {
            new_move = true;
            if player.target_y > 0.0 {
                player.target_y -= 1.0
            }
        }
        else if is_key_pressed(user.down) {
            new_move = true;
            if player.target_y < 15.0 {
                player.target_y += 1.0
            }
        }



        return new_move;
}

#[derive(Debug, Copy, Clone, Savefile)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub target_x: f32,
    pub target_y: f32,
    pub sub_round: u8,
    pub energy: f32,
}

impl Player {

    pub fn update_pos(&mut self) {
        const SPEED: f32 = 0.5;
        if self.target_x > self.x {
            self.x += SPEED
        }
        if self.target_x < self.x {
            self.x -= SPEED
        }
        if self.target_y > self.y {
            self.y += SPEED
        }
        if self.target_y < self.y {
            self.y -= SPEED
        }
        if (self.x - self.target_x).abs() < SPEED {
            self.x = self.target_x
        }
        if (self.y - self.target_y).abs() < SPEED {
            self.y = self.target_y
        }
    }

}