use std::io::Write;

pub struct Game {
    board: Board,
    level: usize,
    player: Player,
    finished: bool,
    level_completed: bool
}

impl Game {
    pub fn new() -> Self {
        let mut ret = Self {
            board: Board {
                cells: Vec::new(),
                level_str: String::new(),
                goal_pos: Pos::from(0, 0),
                crate_pos: Pos::from(0, 0),
            },
            level: 0,
            player: Player {
                pos: Pos::from(1, 1),
                score: 0,
            },
            finished: false,
            level_completed: false
        };

        // Addition setup
        ret.advance_level_string();
        ret.populate_board();

        println!();
        println!("~~ Level {} ~~", ret.level);

        // Returning the setup Game struct
        ret
    }

    pub fn play(&mut self) {
        loop {
            if self.finished {
                println!();
                println!("Congratulations! You've won!");
                break;
            }

            self.print_board();
            println!("Type \'quit\' to quit the game.");
            print!("Where do you want to go? (w, s, a, d): ");
            std::io::stdout().flush().unwrap(); // necessary for print! for whatever reason

            // user input for where they want to go
            let mut direction = String::new();
            match std::io::stdin().read_line(&mut direction) {
                Ok(_) => (),
                Err(_) => panic!("Unable to receive user input."),
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
                }
                _ if direction == Direction::East.char() => {
                    dir = Some(Direction::East);
                }
                _ if direction == Direction::South.char() => {
                    dir = Some(Direction::South);
                }
                _ if direction == Direction::West.char() => {
                    dir = Some(Direction::West);
                }
                _ => {
                    println!();
                    println!("Please enter a valid direction.");
                    continue;
                }
            }

            self.move_object(
                CellType::Player,
                if let None = dir {
                    panic!("Direction \'dir\' nor correctly assigned")
                } else {
                    dir.unwrap()
                },
            );
            self.update_level_string();
        }
    }

    fn move_object(&mut self, obj: CellType, dir: Direction) -> bool {
        let obj_pos = match &obj {
            &CellType::Player => self.player.pos,
            &CellType::Crate => self.board.crate_pos,
            _ => panic!("Attempt to call move_object() with invalid type") // format with obj when Debug implemented for CellType
        };

        let destination: Pos = match dir {
            Direction::North => { 
                if obj_pos.y == 0 {
                    Pos::from(
                        obj_pos.x,
                        self.board.cells.len()-1,
                    )
                } else {
                    Pos::from(
                        obj_pos.x,
                        obj_pos.y-1
                    )
                }
            },
            Direction::South => {
                if obj_pos.y == self.board.cells.len()-1 {
                    Pos::from(
                        obj_pos.x,
                        0
                    )
                } else {
                    Pos::from(
                        obj_pos.x,
                        obj_pos.y+1
                    )
                }
            },
            Direction::East => {
                if obj_pos.x == self.board.cells[obj_pos.y].len()-1 {
                    Pos::from(
                        0,
                        obj_pos.y
                    )
                } else {
                    Pos::from(
                        obj_pos.x+1,
                        obj_pos.y
                    )
                }
            }
            Direction::West => {
                if obj_pos.x == 0 {
                    Pos::from(
                        self.board.cells[obj_pos.y].len()-1,
                        obj_pos.y
                    )
                } else {
                    Pos::from(
                        obj_pos.x-1,
                        obj_pos.y
                    )
                }
            }
        };
        
        // if the destination is a crate, try and move it; if its a goal or floor, go on it, and if it’s a wall, dont move
        // self.update_pos_of(&obj, &destination);
        let can_move = match self.cell_at_pos(&destination).cell_type {
            CellType::Crate => {
                let res = self.move_object(CellType::Crate, dir);
                if self.level_completed {
                    self.level_completed = false;
                    return true // the return value doesn't matter here
                }
                res
            },
            CellType::Wall => false,
            CellType::Goal => {
                //* Win condition here
                if &obj == &CellType::Crate {
                    self.level_complete();
                    self.level_completed = true;
                    return true
                }
                true
            }
            _ => true
        };

        if can_move {
            self.cell_at_pos_mut(&destination).cell_type = obj.clone();
            self.cell_at_pos_mut(&obj_pos).cell_type = if obj_pos == self.board.goal_pos {
                CellType::Goal
            } else {
                CellType::Floor
            };

            self.update_object_pos(&obj, &destination);
        }


        can_move
    }

    fn update_object_pos(&mut self, obj: &CellType, new: &Pos) {
        match obj {
            CellType::Player => self.player.pos = *new,
            CellType::Crate => self.board.crate_pos = *new,
            _ => panic!("Attempt to call update_object_pos() on unsupported CellType")
        }
    }

    fn cell_at_pos(&self, pos: &Pos) -> &Cell {
        &self.board.cells[pos.y][pos.x]
    }

    fn cell_at_pos_mut(&mut self, pos: &Pos) -> &mut Cell {
        &mut self.board.cells[pos.y][pos.x]
    }

    fn level_complete(&mut self) {
        println!();
        println!("Congratulations! you passed level {}!", self.level);

        self.advance_level_string();
        self.populate_board();
        // println!("{}", self.board.level_str);

        if !self.finished {
            println!("Advancing to level {}.", self.level);
            println!();
            println!("~~ Level {} ~~", self.level);
        }
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
        println!("~~ Board ~~");
        println!();

        println!("+{}+", "-".repeat(self.board.cells[0].len()));
        for vec in &self.board.cells {
            let mut tmp = String::new();
            tmp.push('|');
            for cell in vec {
                tmp.push(cell.cell_type.display_char());
            }
            tmp.push('|');

            println!("{}", tmp);
        }
        println!("+{}+", "-".repeat(self.board.cells.last().unwrap().len()));

        println!();

        // for vec in &self.board.cells {
        //     for cell in vec {
        //         print!("{}, ", cell.cell_type.char());
        //         std::io::stdout().flush().unwrap();
        //     }
        //     println!();
        // }
        // println!();
    }

    fn advance_level_string(&mut self) {
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

    fn populate_board(&mut self) {
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
                        self.board.goal_pos = Pos::from(col_ind, row_ind)
                    }
                    _ if col == CellType::Player.char() => {
                        row_vec.push(Cell::new(CellType::Player));
                        self.player.pos = Pos::from(col_ind, row_ind);
                    }
                    _ if col == CellType::Crate.char() => {
                        row_vec.push(Cell::new(CellType::Crate));
                        self.board.crate_pos = Pos::from(col_ind, row_ind);
                    }
                    _ => {
                        panic!(
                            "Unsupported character \'{}\' in level {}.",
                            col, self.level
                        )
                    }
                }
            }
            self.board.cells.push(row_vec);
        }
    }
}

pub struct Player {
    pos: Pos,
    score: isize,
}

pub struct Board {
    cells: Vec<Vec<Cell>>,
    level_str: String,
    goal_pos: Pos,
    crate_pos: Pos,
}

#[derive(Clone, Copy)]
pub struct Pos {
    x: usize,
    y: usize,
}

impl PartialEq for Pos {
    fn eq(&self, other: &Pos) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Pos {
    fn from(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

pub struct Cell {
    cell_type: CellType,
}

impl Cell {
    fn new(cell_type: CellType) -> Self {
        Self { cell_type }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum CellType {
    Wall,
    Floor,
    Crate,
    Player,
    Goal,
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

    fn display_char(&self) -> char { // TODO: Find some unicode characters to use for these
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
    West,
}

impl Direction {
    fn char(&self) -> char {
        match self {
            &Direction::North => 'w',
            &Direction::East => 'd',
            &Direction::South => 's',
            &Direction::West => 'a',
        }
    }
}
