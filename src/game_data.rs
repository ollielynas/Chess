use crate::ability::*;
use crate::home::*;
use crate::particles_fnc::*;
use crate::player::*;
use ::rand::prelude::*;
use macroquad::prelude::*;
extern crate savefile;

#[derive(Debug, Copy, Clone, PartialEq, Savefile)]
pub enum Piece {
    Pawn,
    Knight,
    Rook,
    Bishop,
    Queen,
    King(f32),
}

#[derive(Debug, Copy, Clone, PartialEq, Savefile, Default)]
pub struct Coord {
    pub x: f32,
    pub y: f32,
}
#[derive(Debug, Clone, PartialEq, Savefile)]
pub struct Enemy {
    pub x: f32,
    pub y: f32,
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
pub struct SelectSquare {
    pub point: Coord,
    pub select_mode: bool,
    pub read: bool,
    pub ability: Abilities,
}

impl Default for SelectSquare {
    fn default() -> SelectSquare {
        SelectSquare {
            point: Coord { x: 0.0, y: 0.0 },
            read: false,
            select_mode: false,
            ability: Abilities::Null,
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
    #[savefile_ignore]
    pub select_square: SelectSquare,
    pub score: f32,
    pub score_text: Vec<TextReadout>,
    pub effects: Vec<(Abilities, f32)>,
    #[savefile_ignore]
    pub select_texture_pack: bool,
    #[savefile_ignore]
    pub select_keybinds: bool,
    #[savefile_ignore]
    pub keybind_focus: f32,
}

impl GameData {
    pub fn difficulty(&self) -> f32 {
        (self.round as f32 / 3.141569).sin() + (self.round as f32 / 40.0)
    }


    pub fn enemy_move(&mut self) {
        if self.enemies.is_empty() {
            println!("no enemies");
            return;
        }

        let mut filled: Vec<Coord> = vec![Coord { x: -10.0, y: -10.0 }];
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
                        if i.moves.len() < 1 {
                            break;
                        };
                        if (j.x == i.moves[i.moves.len() - 1].x
                            && j.y == i.moves[i.moves.len() - 1].y)
                            || (i.moves[i.moves.len() - 1].x > 15.0
                                || i.moves[i.moves.len() - 1].x < 0.0
                                || i.moves[i.moves.len() - 1].y > 15.0
                                || i.moves[i.moves.len() - 1].y < 0.0)
                        {
                            // break;
                            i.moves.remove(i.moves.len() - 1)
                        } else {
                            break;
                        };
                    }
                }
                if !i.moves.is_empty() {
                    i.x = i.moves[i.moves.len() - 1].x;
                    i.y = i.moves[i.moves.len() - 1].y;
                    filled.push(Coord { x: i.x, y: i.y });
                }

                let x_dist = self.player.target_x - i.x as f32;
                let y_dist = self.player.target_y - i.y as f32;

                fn bishop_intersect(_player: Coord, bishop: Coord) -> Vec<Coord> {
                    vec![Coord {
                        x: bishop.x,
                        y: bishop.y,
                    }]
                }

                i.moves = match i.piece {
                    // the move list should include the tile the piece is standing on
                    // ||------------------------------ Pawn AI -------------------------------|
                    Piece::Pawn => {
                        if (x_dist).abs() > y_dist.abs() {
                            if x_dist < 0.0 {
                                vec![
                                    Coord {
                                        x: i.x as f32,
                                        y: i.y as f32,
                                    },
                                    Coord {
                                        x: i.x as f32 - 1.0,
                                        y: i.y as f32,
                                    },
                                ]
                            } else {
                                vec![
                                    Coord {
                                        x: i.x as f32,
                                        y: i.y as f32,
                                    },
                                    Coord {
                                        x: i.x as f32 + 1.0,
                                        y: i.y as f32,
                                    },
                                ]
                            }
                        } else {
                            if y_dist < 0.0 {
                                vec![
                                    Coord {
                                        x: i.x as f32,
                                        y: i.y as f32,
                                    },
                                    Coord {
                                        x: i.x as f32,
                                        y: i.y as f32 - 1.0,
                                    },
                                ]
                            } else {
                                vec![
                                    Coord {
                                        x: i.x as f32,
                                        y: i.y as f32,
                                    },
                                    Coord {
                                        x: i.x as f32,
                                        y: i.y as f32 + 1.0,
                                    },
                                ]
                            }
                        }
                    }
                    // ||------------------------------ Rook AI -------------------------------||
                    Piece::Rook => {
                        if (x_dist).abs() > y_dist.abs() {
                            if x_dist < 0.0 {
                                (0..x_dist.abs() as usize + 1)
                                    .map(|e| Coord {
                                        x: i.x as f32 - e as f32,
                                        y: i.y as f32,
                                    })
                                    .collect()
                            } else {
                                (0..x_dist.abs() as usize + 1)
                                    .map(|e| Coord {
                                        x: i.x as f32 + e as f32,
                                        y: i.y as f32,
                                    })
                                    .collect()
                            }
                        } else {
                            if y_dist < 0.0 {
                                (0..y_dist.abs() as usize + 1)
                                    .map(|e| Coord {
                                        x: i.x as f32,
                                        y: i.y as f32 - e as f32,
                                    })
                                    .collect()
                            } else {
                                (0..y_dist.abs() as usize + 1)
                                    .map(|e| Coord {
                                        x: i.x as f32,
                                        y: i.y as f32 + e as f32,
                                    })
                                    .collect()
                            }
                        }
                    }

                    Piece::Knight => {
                        if (x_dist).abs() > y_dist.abs() {
                            if x_dist < 0.0 {
                                (0..3)
                                    .map(|e| Coord {
                                        x: i.x as f32 - e as f32,
                                        y: i.y as f32,
                                    })
                                    .collect()
                            } else {
                                (0..3)
                                    .map(|e| Coord {
                                        x: i.x as f32 + e as f32,
                                        y: i.y as f32,
                                    })
                                    .collect()
                            }
                        } else {
                            if y_dist < 0.0 {
                                (0..3)
                                    .map(|e| Coord {
                                        x: i.x as f32,
                                        y: i.y as f32 - e as f32,
                                    })
                                    .collect()
                            } else {
                                (0..3)
                                    .map(|e| Coord {
                                        x: i.x as f32,
                                        y: i.y as f32 + e as f32,
                                    })
                                    .collect()
                            }
                        }
                    }

                    Piece::Bishop => {
                        if true || y_dist.abs() == x_dist.abs() {
                            if x_dist < 0.0 {
                                if y_dist < 0.0 {
                                    (0..y_dist.abs() as usize + thread_rng().gen_range(2..8))
                                        .map(|e| Coord {
                                            x: i.x as f32 - e as f32,
                                            y: i.y as f32 - e as f32,
                                        })
                                        .collect()
                                } else {
                                    (0..y_dist.abs() as usize + thread_rng().gen_range(2..8))
                                        .map(|e| Coord {
                                            x: i.x as f32 - e as f32,
                                            y: i.y as f32 + e as f32,
                                        })
                                        .collect()
                                }
                            } else {
                                if y_dist < 0.0 {
                                    (0..y_dist.abs() as usize + thread_rng().gen_range(2..8))
                                        .map(|e| Coord {
                                            x: i.x as f32 + e as f32,
                                            y: i.y as f32 - e as f32,
                                        })
                                        .collect()
                                } else {
                                    (0..y_dist.abs() as usize + thread_rng().gen_range(2..8))
                                        .map(|e| Coord {
                                            x: i.x as f32 + e as f32,
                                            y: i.y as f32 + e as f32,
                                        })
                                        .collect()
                                }
                            }
                        } else {
                            bishop_intersect(
                                Coord {
                                    x: self.player.target_x,
                                    y: self.player.target_y,
                                },
                                Coord { x: i.x, y: i.y },
                            )
                        }
                    }
                    Piece::Queen => {
                        if y_dist.abs() == x_dist.abs() {
                            if x_dist < 0.0 {
                                if y_dist < 0.0 {
                                    (0..y_dist.abs() as usize + thread_rng().gen_range(2..8))
                                        .map(|e| Coord {
                                            x: i.x as f32 - e as f32,
                                            y: i.y as f32 - e as f32,
                                        })
                                        .collect()
                                } else {
                                    (0..y_dist.abs() as usize + thread_rng().gen_range(2..8))
                                        .map(|e| Coord {
                                            x: i.x as f32 - e as f32,
                                            y: i.y as f32 + e as f32,
                                        })
                                        .collect()
                                }
                            } else {
                                if y_dist < 0.0 {
                                    (0..y_dist.abs() as usize + thread_rng().gen_range(2..8))
                                        .map(|e| Coord {
                                            x: i.x as f32 + e as f32,
                                            y: i.y as f32 - e as f32,
                                        })
                                        .collect()
                                } else {
                                    (0..y_dist.abs() as usize + thread_rng().gen_range(2..8))
                                        .map(|e| Coord {
                                            x: i.x as f32 + e as f32,
                                            y: i.y as f32 + e as f32,
                                        })
                                        .collect()
                                }
                            }
                        } else {
                            if (x_dist).abs() > y_dist.abs() {
                                if x_dist < 0.0 {
                                    (0..x_dist.abs() as usize + 1)
                                        .map(|e| Coord {
                                            x: i.x as f32 - e as f32,
                                            y: i.y as f32,
                                        })
                                        .collect()
                                } else {
                                    (0..x_dist.abs() as usize + 1)
                                        .map(|e| Coord {
                                            x: i.x as f32 + e as f32,
                                            y: i.y as f32,
                                        })
                                        .collect()
                                }
                            } else {
                                if y_dist < 0.0 {
                                    (0..y_dist.abs() as usize + 1)
                                        .map(|e| Coord {
                                            x: i.x as f32,
                                            y: i.y as f32 - e as f32,
                                        })
                                        .collect()
                                } else {
                                    (0..y_dist.abs() as usize + 1)
                                        .map(|e| Coord {
                                            x: i.x as f32,
                                            y: i.y as f32 + e as f32,
                                        })
                                        .collect()
                                }
                            }
                        }
                    },
                    
                    Piece::King(_) => {
                        if (x_dist).abs() > y_dist.abs() {
                            if x_dist < 0.0 {
                                vec![
                                    Coord {
                                        x: i.x as f32,
                                        y: i.y as f32,
                                    },
                                    Coord {
                                        x: i.x as f32 - 1.0,
                                        y: i.y as f32,
                                    },
                                ]
                            } else {
                                vec![
                                    Coord {
                                        x: i.x as f32,
                                        y: i.y as f32,
                                    },
                                    Coord {
                                        x: i.x as f32 + 1.0,
                                        y: i.y as f32,
                                    },
                                ]
                            }
                        } else {
                            if y_dist < 0.0 {
                                vec![
                                    Coord {
                                        x: i.x as f32,
                                        y: i.y as f32,
                                    },
                                    Coord {
                                        x: i.x as f32,
                                        y: i.y as f32 - 1.0,
                                    },
                                ]
                            } else {
                                vec![
                                    Coord {
                                        x: i.x as f32,
                                        y: i.y as f32,
                                    },
                                    Coord {
                                        x: i.x as f32,
                                        y: i.y as f32 + 1.0,
                                    },
                                ]
                            }
                        }
                    }
                    // ||------------------------------ Everything else AI ----------------------||

                };
                i.moves.retain(|&f| f.x < 16.0 && f.y < 16.0);
                i.moves.retain(|&f| f.x >= 0.0 && f.y >= 0.0);

                match i.piece {
                    Piece::Knight => {
                        if !i.moves.is_empty() {
                            if (x_dist).abs() > y_dist.abs() {
                                if y_dist < 0.0 {
                                    i.moves.push(Coord {
                                        x: i.moves[i.moves.len() - 1].x,
                                        y: i.y - 1.0,
                                    });
                                } else {
                                    i.moves.push(Coord {
                                        x: i.moves[i.moves.len() - 1].x,
                                        y: i.y + 1.0,
                                    });
                                }
                            } else {
                                if x_dist < 0.0 {
                                    i.moves.push(Coord {
                                        x: i.x as f32 - 1.0,
                                        y: i.moves[i.moves.len() - 1].y,
                                    });
                                } else {
                                    i.moves.push(Coord {
                                        x: i.x as f32 + 1.0,
                                        y: i.moves[i.moves.len() - 1].y,
                                    });
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    pub fn spawn_enemy(&mut self, single: bool) {

        let mut d = self.difficulty().ceil();

        for i in &self.effects {
            if i.0 == Abilities::Peaceful {
                d -= 5.0;
                break
            }
        }



        if d < 0.0 {
            d = 0.0;
        }

        if single {d = 1.0};

        
        for _ in 0..d as usize {
            let data = thread_rng().gen_range(0..15);
            let chance = thread_rng().gen_range(0..4);
            let spawn_coords = match chance {
                0 => (0, data),
                1 => (15, data),
                2 => (data, 0),
                _ => (data, 15),
            };
            let mut piece_type = Piece::Pawn;
            if self.round > 10 && thread_rng().gen_bool(0.25) {
                piece_type = Piece::Rook;
            }
            if self.round > 10 && thread_rng().gen_bool(0.25) {
                piece_type = Piece::Knight;
            }
            if self.round > 25 && thread_rng().gen_bool(0.25) {
                piece_type = Piece::Bishop;
            }
            if self.round > 50 && thread_rng().gen_bool(0.15) {
                piece_type = Piece::Queen;
            }
            if self.round > 75 && thread_rng().gen_bool(0.15) {
                piece_type = Piece::King((d/2.0).round());
            }

            self.enemies.push(Enemy {
                x: -20.0 * (8 - spawn_coords.0) as f32,
                y: -20.0 * (8 - spawn_coords.1) as f32,
                piece: piece_type,
                moves: vec![
                    Coord {
                        x: -20.0 * (8 - spawn_coords.0) as f32,
                        y: -20.0 * (8 - spawn_coords.1) as f32,
                    },
                    Coord {
                        x: -20.0 * (8 - spawn_coords.0) as f32,
                        y: -20.0 * (8 - spawn_coords.1) as f32,
                    },
                    Coord {
                        x: spawn_coords.0 as f32,
                        y: spawn_coords.1 as f32,
                    },
                ],
            });
        }
    }
}
