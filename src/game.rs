use std::io::Write;

pub struct Game {
    board: Board,
    level: usize,
    player: Player,
    finished: bool
}

impl Game {
    pub fn new() -> Result<Self, String> {
        let mut ret = Self {
            board: Board {
                cells: Vec::new(),
                level_str: String::new(),
                goal_location: Pos::from(0, 0),
                crate_location: Pos::from(0, 0)
            },
            level: 0,
            player: Player {
                x: 1,
                y: 1,
                score: 0
            },
            finished: false
        };

        // Addition setup
        ret.advance_level();
        ret.populate_board()?;

        // Returning the setup Game struct
        Ok(ret)
    }

    pub fn play(&mut self) -> Result<(), String> {
        loop {
            if self.finished {
                println!("Congratulations! You've won!");
                break
            }

            self.print_board();
            println!("Type \'quit\' to quit the game.");
            print!("Where do you want to go? (n, e, s, w): ");
            std::io::stdout().flush().unwrap(); // necessary for print! for whatever reason

            // user input for where they want to go
            let mut direction = String::new();
            match std::io::stdin().read_line(&mut direction) {
                Ok(_) => (),
                Err(_) => return Err(String::from("Unable to receive user input."))
            };

            let direction = direction.trim();

            // println!("direction: \'{}\'", direction);

            // If the user wants to stop
            if direction == "quit" {
                println!();
                println!("Quitting...");
                break;
            }

            if direction.len() != 1 {
                println!();
                println!("Please enter a valid direction.");
                continue;
            }

            let direction = match direction.chars().nth(0) {
                Some(val) => val,
                None => {
                    println!();
                    println!("Please enter a valid direction.");
                    continue;
                }
            };

            let mut dir: Option<Direction> = None;

            match direction {
                _ if direction == Direction::North.char() => {
                    dir = Some(Direction::North);
                },
                _ if direction == Direction::East.char() => {
                    dir = Some(Direction::East);
                },
                _ if direction == Direction::South.char() => {
                    dir = Some(Direction::South);
                },
                _ if direction == Direction::West.char() => {
                    dir = Some(Direction::West);
                },
                _ => {
                    println!();
                    println!("Please enter a valid direction.");
                    continue;
                }
            }

            self.move_player(
            if let None = dir {
                    return Err(String::from("Direction \'dir\' nor correctly assigned"))
                } else {
                    dir.unwrap()
                }
            )?;
        }

        Ok(())
    }

    fn level_complete(&mut self) -> Result<(), String> {
        self.advance_level();
        self.populate_board()?;
        println!("{}", self.board.level_str);
        Ok(())
    }

    fn move_player(&mut self, dir: Direction) -> Result<(), String> {
        // println!("player position \'{}:{}\'", self.player.x, self.player.y);
        match dir {
            Direction::North => {
                // if the player is at the edge, try to wrap them to the top
                if self.player.y == 0 {
                    
                    let final_row_ind = self.board.cells.len()-1;
                    
                    match self.board.cells[final_row_ind][self.player.x].cell_type {
                        CellType::Floor => {
                            self.wrap_player_north();
                        },
                        CellType::Crate => {
                            if self.move_crate(Direction::North)? {
                                self.wrap_player_north();
                            }
                        },
                        CellType::Goal => {
                            self.wrap_player_north();
                        },
                        _ => ()
                    }
                } else {        
                    // println!("Hello");
                    
                    match self.board.cells[self.player.y-1][self.player.x].cell_type {
                        CellType::Floor => {
                            self.move_player_north();
                        },
                        CellType::Crate => {
                            // println!("ello");
                            if self.move_crate(Direction::North)? {
                                // println!("we can move the crate!");
                                self.move_player_north();
                            } else {
                                // println!("we cannot move the crate silly");
                            }
                        },
                        CellType::Goal => {
                            self.move_player_north();
                        },
                        _ => ()
                    }
                }
            },
            Direction::South => (),
            Direction::East => (),
            Direction::West => ()
        }
        
        // this allows the player to cross over the goal without causing issues
        let player_pos = (self.player.x, self.player.y);
        let goal_location = self.board.goal_location.clone();
        if player_pos.0 != goal_location.x || player_pos.1 != goal_location.y {
            self.board.cells[goal_location.y][goal_location.x].cell_type = CellType::Goal;
        }

        self.update_level_string();

        Ok(())
    }

    fn move_player_north(&mut self) {
        let mut destination = &mut self.board.cells[self.player.y-1][self.player.x];

        destination.cell_type = CellType::Player;
        self.board.cells[self.player.y][self.player.x].cell_type = CellType::Floor;
        self.player.y -= 1;

        // self.update_level_string()
    }

    fn wrap_player_north(&mut self) {
        let final_row_ind = self.board.cells.len()-1;
        let mut destination_cell = &mut self.board.cells[final_row_ind][self.player.x];

        destination_cell.cell_type = CellType::Player;
        self.board.cells[self.player.y][self.player.x].cell_type = CellType::Floor;
        self.player.y = final_row_ind;
    }

    fn move_crate(&mut self, direction: Direction) -> Result<bool, String> {
        // at the moment, crates cannot be wrapped

        // println!("crate: {}:{}", self.board.crate_location.x, self.board.crate_location.y);

        let mut dest_pos = Pos::from(0, 0);

        let destination = match direction {
            Direction::North => {
                if self.board.crate_location.y == 0 { return Ok(false) }

                // println!("ello");

                dest_pos = Pos::from(self.board.crate_location.x, self.board.crate_location.y-1);

                // println!("ello");

                &mut self.board.cells[dest_pos.y][dest_pos.x]
            },
            Direction::South => {
                if self.board.crate_location.y == self.board.cells.len()-1 { return Ok(false) }

                dest_pos = Pos::from(self.board.crate_location.x, self.board.crate_location.y+1);

                &mut self.board.cells[dest_pos.y][dest_pos.x]
            },
            Direction::East => {
                if self.board.crate_location.x == self.board.cells.len()-1 { return Ok(false) }

                dest_pos = Pos::from(self.board.crate_location.x+1, self.board.crate_location.y);

                &mut self.board.cells[dest_pos.y][dest_pos.x]
            },
            Direction::West => {
                if self.board.crate_location.x == 0 { return Ok(false) }

                dest_pos = Pos::from(self.board.crate_location.x-1, self.board.crate_location.y);

                &mut self.board.cells[dest_pos.y][dest_pos.x]
            }
        };

        // println!("destination: {:?}", destination.cell_type);
        
        let should_move = match destination.cell_type {
            CellType::Wall => false,
            CellType::Goal => {
                self.level_complete()?;
                return Ok(true)
            },
            _ => true
        };

        if should_move {
            destination.cell_type = CellType::Crate;
            self.board.cells[self.board.crate_location.y][self.board.crate_location.x].cell_type = CellType::Floor;
            self.board.crate_location = dest_pos;
        }

        // println!("crate location {}:{}", self.board.crate_location.x, self.board.crate_location.y);

        // self.update_level_string();

        Ok(should_move)
    }

    fn update_level_string(&mut self) {
        let mut tmp = String::new();
        for row in &self.board.cells {
            for cell in row {
                tmp.push(cell.cell_type.char())
            }
            tmp.push('\n');
        }
        self.board.level_str = tmp;
    }

    fn print_board(&self) {
        println!();
        println!("Board:");
        println!("{}", self.board.level_str);
        println!();
    }

    fn advance_level(&mut self) {
        self.level += 1;
        self.board.level_str = match std::fs::read_to_string(format!("levels/{}.txt", self.level)) {
            Ok(string) => string,
            Err(_) => {
                self.finished = true;
                // println!("finished!");
                String::from("#")
            }
        };
    }

    fn populate_board(&mut self) -> Result<(), String> {
        // Reset the cells
        self.board.cells = Vec::new();

        for (row_ind, row) in self.board.level_str.lines().enumerate() {
            let mut row_vec: Vec<Cell> = Vec::new();
            for (col_ind, col) in row.chars().enumerate() {
                match col {
                    _ if col == CellType::Wall.char() => row_vec.push(Cell::new(CellType::Wall)),
                    _ if col == CellType::Floor.char() => row_vec.push(Cell::new(CellType::Floor)),
                    _ if col == CellType::Goal.char() => {
                        row_vec.push(Cell::new(CellType::Goal));
                        self.board.goal_location = Pos::from(col_ind, row_ind)
                    },
                    _ if col == CellType::Player.char() => {
                        row_vec.push(Cell::new(CellType::Player));
                        self.player.x = col_ind;
                        self.player.y = row_ind;
                        // println!("Player pos x:{} and y:{}", self.player.x, self.player.y);
                    },
                    _ if col == CellType::Crate.char() => {
                        row_vec.push(Cell::new(CellType::Crate));
                        self.board.crate_location = Pos::from(col_ind, row_ind);
                    },
                    _ => return Err(format!("Unsupported character \'{}\' in level {}.", col, self.level))
                }
            }
            self.board.cells.push(row_vec);
        }
        Ok(())
    }
}

pub struct Player {
    x: usize,
    y: usize,
    score: isize,
}

pub struct Board {
    cells: Vec<Vec<Cell>>,
    level_str: String,
    goal_location: Pos,
    crate_location: Pos
}

#[derive(Clone, Copy)]
pub struct Pos {
    x: usize,
    y: usize
}

impl Pos {
    fn from(x: usize, y: usize) -> Self {
        Self {
            x,
            y
        }
    }
}

pub struct Cell {
    cell_type: CellType,
}

impl Cell {
    fn new(cell_type: CellType) -> Self {
        Self {
            cell_type
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum CellType {
    Wall,
    Floor,
    Crate,
    Player,
    Goal
}

impl CellType {
    fn char(&self) -> char {
        match self {
            &CellType::Wall => '#',
            &CellType::Floor => ' ',
            &CellType::Crate => 'C',
            &CellType::Player => 'P',
            &CellType::Goal => 'G',
        }
    }
}

enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    fn char(&self) -> char {
        match self {
            &Direction::North => 'n',
            &Direction::East => 'e',
            &Direction::South => 's',
            &Direction::West => 'w'
        }
    }
}