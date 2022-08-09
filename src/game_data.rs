use crate::player::*;
use macroquad::{prelude::*};
use ::rand::prelude::*;


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Piece {
    Pawn,
    Knight,
    Rook,
    Bishop,
    Queen,
    King
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Coord {
    pub x: f32,
    pub y: f32,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Enemy {
    pub x:f32,
    pub y:f32,
    pub piece: Piece,
    pub moves: Vec<Coord>,
}

pub struct GameData {
    pub player: Player,
    pub round: u64,
    pub enemies: Vec<Enemy>,
    pub alive: bool,
}


impl GameData {
    pub fn enemy_move(&mut self) {
        if self.enemies.len() < 1 {return}
        let mut filled: Vec<Coord> = vec![];
        for i in &mut self.enemies {


            if i.moves.len() < 1 {return}
            // loop through the moves being made
            for l in &i.moves {
                // kills player if hit
                if l.x == self.player.target_x && l.y == self.player.target_y {
                    self.alive = false;
                }
            }

            // move to final space
            for j in &filled {
                loop {
                if i.moves.len() < 1 {break};
                if j.x == i.moves[i.moves.len() - 1].x && j.y == i.moves[i.moves.len() - 1].y {
                    i.moves.remove(i.moves.len() - 1)
                }else {break};
            }
            }

            if i.moves.len() > 0 {
                i.x = i.moves[i.moves.len() - 1].x;
                i.y = i.moves[i.moves.len() - 1].y;
            }
            filled.push(Coord {x: i.x, y: i.y});
            
            let x_dist = self.player.x - i.x as f32;
            let y_dist = self.player.y - i.y as f32;

            
            i.moves = match i.piece {
                // the move list should include the tile the piece is standing on
// ||------------------------------ Pawn AI -------------------------------|
            Piece::Pawn => (
                if (x_dist).abs() > y_dist.abs() {
                    if x_dist < 0.0 {
                        vec![
                            Coord {x: i.x as f32, y: i.y as f32},
                            Coord {x: i.x as f32 -1.0, y: i.y as f32}]
                    }else {
                        vec![
                            Coord {x: i.x as f32, y: i.y as f32},
                            Coord {x: i.x as f32 +1.0, y: i.y as f32}]
                    }
                }else {
                    if y_dist < 0.0 {
                        vec![Coord {x: i.x as f32, y: i.y as f32},
                        Coord {x: i.x as f32 , y: i.y as f32 -1.0 }]
                    }else {
                        vec![Coord {x: i.x as f32, y: i.y as f32},
                        Coord {x: i.x as f32, y: i.y as f32 +1.0 }]
                    }
                }
            ),
// ||------------------------------ Rook AI -------------------------------||

            Piece::Rook => (
                if (x_dist).abs() > y_dist.abs() {
                    if x_dist < 0.0 {
                        (0..x_dist.abs() as usize +1).map(|e| Coord {x: i.x as f32 -e as f32, y: i.y as f32}).collect()

                    }else {
                        (0..x_dist.abs() as usize +1).map(|e| Coord {x: i.x as f32 +e as f32, y: i.y as f32}).collect()
                    }
                }else {
                    if y_dist < 0.0 {
                        (0..y_dist.abs() as usize +1).map(|e| Coord {x: i.x as f32 , y: i.y as f32 -e as f32}).collect()
                    }else {
                        (0..y_dist.abs() as usize +1).map(|e| Coord {x: i.x as f32 , y: i.y as f32 +e as f32}).collect()
                    }
                }
            ),
// ||------------------------------ Everything else AI ----------------------||
            _ => vec![]
        };
        }
        // this is meant to stop the pieces colliding
        // if self.enemies.len() <= 1 {return};
        // for i in 0..&self.enemies.len()-1 {
        //     for j in self.enemies.clone() {
        //         loop {
        //         if self.enemies[i].moves.len() > 0 && j.moves.len() > 0 && self.enemies[i] != j {
        //             let index = self.enemies[i].moves.len()-1;
        //             if self.enemies[i].moves[index] == j.moves[j.moves.len()-1] {
        //                 self.enemies[i].moves.remove(index);
        //             }else {
        //                 break;
        //             }
        //         } else {break}
        //     }
        //     }
        // }

    }
    pub fn spawn_enemy(&mut self) {

        let data = thread_rng().gen_range(0..15);
        let chance = thread_rng().gen_range(0..4);
        let spawn_coords = match chance {
            0 => (0, data),
            1 => (15, data),
            2 => (data, 0),
            _ => (data, 15),
        };

        let mut piece_type = Piece::Pawn;
        if self.round > 10 && thread_rng().gen_bool(0.5+self.round as f64/200.0) {
            piece_type = Piece::Rook;
        }

        self.enemies.push(Enemy {
            x: 20.0*(8-spawn_coords.0) as f32,
            y: 20.0*(8-spawn_coords.1) as f32,
            piece: piece_type,
            moves: vec![Coord {x:spawn_coords.0 as f32, y: spawn_coords.1 as f32}]
        });
        
    }
}



pub struct UserData {
    pub left: KeyCode,
    pub right: KeyCode,
    pub up: KeyCode,
    pub down: KeyCode,
}