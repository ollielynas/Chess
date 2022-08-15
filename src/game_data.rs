use crate::player::*;
use macroquad::{prelude::*};
use ::rand::prelude::*;
use crate::particles_fnc::*;

extern crate savefile;



#[derive(Debug, Copy, Clone, PartialEq, Savefile)]
pub enum Piece {
    Pawn,
    Knight,
    Rook,
    Bishop,
    Queen,
    King
}

#[derive(Debug, Copy, Clone, PartialEq, Savefile)]
pub struct Coord {
    pub x: f32,
    pub y: f32,
}
#[derive(Debug, Clone, PartialEq, Savefile)]
pub struct Enemy {
    pub x:f32,
    pub y:f32,
    pub piece: Piece,
    pub moves: Vec<Coord>,
}

#[derive(Savefile)]
pub struct SelectAbility {
    pub slot: usize,
    pub open: bool,
    pub page: usize,
}

impl Default for SelectAbility {
    fn default() -> SelectAbility {
        SelectAbility {
            slot: 0,
            open: false,
            page: 0,
        }
    }
}

#[derive(Savefile)]
pub struct GameData {
    pub player: Player,
    pub round: u64,
    pub enemies: Vec<Enemy>,
    pub alive: bool,
    pub bubble_particles: Vec<Bubble>,
    #[savefile_ignore]
    pub select_ability: SelectAbility,
    #[savefile_ignore]
    pub pause: bool,
}


impl GameData {

    pub fn enemy_move(&mut self) {
        if self.enemies.len() < 1 {
            println!("no enemies");
            return
        }

        let mut filled: Vec<Coord> = vec![
            Coord {x: -10.0, y: -10.0}
        ];
        for i in &mut self.enemies {
            
            if i.moves.len() > 1 {
            
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
                if (j.x == i.moves[i.moves.len() - 1].x && j.y == i.moves[i.moves.len() - 1].y) 
                ||(i.moves[i.moves.len() - 1].x > 15.0 || i.moves[i.moves.len() - 1].x < 0.0 || 
                    i.moves[i.moves.len() - 1].y > 15.0 ||i.moves[i.moves.len() - 1].y < 0.0
            )
                {   
                    // break;
                    i.moves.remove(i.moves.len() - 1)
                }else {break};
                
            }
            }
            if i.moves.len() > 0 {
                i.x = i.moves[i.moves.len() - 1].x;
                i.y = i.moves[i.moves.len() - 1].y;
                filled.push(Coord {x: i.x, y: i.y});
            }
            
            let x_dist = self.player.target_x - i.x as f32;
            let y_dist = self.player.target_y - i.y as f32;

            fn bishop_intersect(player: Coord, bishop: Coord) -> Vec<Coord> {
                let mut player_line_2:Vec<Coord> = vec![];
                let mut player_line_1:Vec<Coord> = vec![];
                let mut bishop_line_1:Vec<Coord> = vec![];
                let mut  bishop_line_2:Vec<Coord> = vec![];

                for e in -16..16 {
                    if player.x + e as f32 >= 0.0
                    && player.y + e as f32 >= 0.0
                    && player.x + e as f32 <=15.0
                    && player.y + e as f32 <= 15.0
                        {
                            player_line_1.push(Coord {x:player.x +e as f32, y:player.y + e as f32});
                        };
                    if player.x - e as f32 >= 0.0
                    && player.y + e as f32 >= 0.0
                    && player.x - e as f32 <=15.0
                    && player.y + e as f32 <= 15.0
                        {
                            player_line_2.push(Coord {x:player.x -e as f32, y:player.y + e as f32});
                        };
                    if bishop.x - e as f32 >= 0.0
                    && bishop.y + e as f32 >= 0.0
                    && bishop.x - e as f32 <=15.0
                    && bishop.y + e as f32 <= 15.0
                        {
                            player_line_2.push(Coord {x:bishop.x -e as f32, y:bishop.y + e as f32});
                        };
                    if bishop.x + e as f32 >= 0.0
                    && bishop.y + e as f32 >= 0.0
                    && bishop.x + e as f32 <=15.0
                    && bishop.y + e as f32 <= 15.0
                        {
                            player_line_2.push(Coord {x:bishop.x +e as f32, y:bishop.y + e as f32});
                        };
                }



                return vec![]
            }

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

            Piece::Bishop => (
                if y_dist.abs() == x_dist.abs() {
                    if x_dist < 0.0 {
                        if y_dist < 0.0 {
                            (0..y_dist.abs() as usize +thread_rng().gen_range(1..8)).map(|e| Coord {x: i.x as f32 -e as f32 , y: i.y as f32 - e as f32}).collect()
                        }else {
                            (0..y_dist.abs() as usize +thread_rng().gen_range(1..8)).map(|e| Coord {x: i.x as f32 -e as f32 , y: i.y as f32 + e as f32}).collect()
                        }
                    }else {
                        if y_dist < 0.0 {
                            (0..y_dist.abs() as usize +thread_rng().gen_range(1..8)).map(|e| Coord {x: i.x as f32 +e as f32 , y: i.y as f32 - e as f32}).collect()
                        }else {
                            (0..y_dist.abs() as usize +thread_rng().gen_range(1..8)).map(|e| Coord {x: i.x as f32 +e as f32 , y: i.y as f32 + e as f32}).collect()
                        }
                    }
                }else{
                    bishop_intersect(Coord {x: self.player.target_x, y:self.player.target_y}, 
                        Coord {x:i.x, y:i.y}
                    )
                }
            )
            ,
// ||------------------------------ Everything else AI ----------------------||
            _ => vec![Coord {x: i.x as f32 , y: i.y as f32}]
        };
        i.moves.retain(|&f| f.x < 16.0 && f.y < 16.0);
        i.moves.retain(|&f| f.x >= 0.0 && f.y >= 0.0);
        }
    }


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
        if self.round > 10 && thread_rng().gen_bool(0.5) {
            piece_type = Piece::Rook;
        }
        if self.round > 25 && thread_rng().gen_bool(0.5) {
            piece_type = Piece::Bishop;
        }

        self.enemies.push(Enemy {
            x: -20.0*(8-spawn_coords.0) as f32,
            y: -20.0*(8-spawn_coords.1) as f32,
            piece: piece_type,
            moves: vec![
                Coord {x: -20.0*(8-spawn_coords.0) as f32, y:-20.0*(8-spawn_coords.1) as f32},
                Coord {x: -20.0*(8-spawn_coords.0) as f32, y:-20.0*(8-spawn_coords.1) as f32},
                Coord {x:spawn_coords.0 as f32, y: spawn_coords.1 as f32}
                ]
        });
        
    }
}



