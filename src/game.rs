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
                level_str: String::new()
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

            // If the user wants to stop
            if direction == "quit" {
                println!();
                println!("Quitting...");
                break;
            }

            let direction = match direction.chars().nth(0) {
                Some(val) => val,
                None => {
                    println!("Please enter a valid direction.");
                    continue;
                }
            };

            let mut dir = Direction::North;

            match direction {
                _ if direction == Direction::North.char() => {
                    dir = Direction::North;
                },
                _ if direction == Direction::East.char() => {
                    dir = Direction::East;
                },
                _ if direction == Direction::South.char() => {
                    dir = Direction::South;
                },
                _ if direction == Direction::West.char() => {
                    dir = Direction::West;
                },
                _ => {
                    println!("Please enter a valid direction.");
                    continue;
                }
            }

            self.move_player(dir);
        }
        Ok(())
    }

    fn move_player(&mut self, dir: Direction) {
        match dir {
            Direction::North => {
                // if the player is at the edge, try to wrap them
                if self.player.y == 0 {
                    let final_row_ind = self.board.cells.len()-1;
                    // if the player can be wrapped, do so
                    if self.board.cells[final_row_ind][self.player.x].cell_type == CellType::Floor {
                        self.board.cells[final_row_ind][self.player.x].cell_type = CellType::Player;
                        self.board.cells[self.player.y][self.player.x].cell_type = CellType::Floor;
                        self.player.y = final_row_ind;
                    }
                }
            },
            Direction::South => (),
            Direction::East => (),
            Direction::West => ()
        }
        
        self.update_level_string();
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
                println!("finished!");
                String::from("finished")
            }
        };
    }

    fn populate_board(&mut self) -> Result<(), String>{
        for (row_ind, row) in self.board.level_str.lines().enumerate() {
            let mut row_vec: Vec<Cell> = Vec::new();
            for (i_ind, i) in row.chars().enumerate() {
                match i {
                    _ if i == CellType::Wall.char() => row_vec.push(Cell::new(CellType::Wall)),
                    _ if i == CellType::Floor.char() => row_vec.push(Cell::new(CellType::Floor)),
                    _ if i == CellType::Goal.char() => row_vec.push(Cell::new(CellType::Goal)),
                    _ if i == CellType::Player.char() => {
                        row_vec.push(Cell::new(CellType::Player));
                        self.player.x = i_ind;
                        self.player.y = row_ind;
                        // println!("Player pos x:{} and y:{}", self.player.x, self.player.y);
                    },
                    _ if i == CellType::Crate.char() => row_vec.push(Cell::new(CellType::Crate)),
                    _ => return Err(format!("Unsupported character \'{}\' in level {}.", i, self.level))
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

#[derive(PartialEq)]
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