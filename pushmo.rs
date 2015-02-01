// pushmo.rs

use std::char::CharExt;
use std::io::File;
use std::io::Lines;
use std::io::BufferedReader;
use std::collections::VecMap;
use std::collections::HashMap;
use std::char;
use std::iter;
use std::hash;
use std::fmt;
use std::os;

// Point
struct Point {
    x: i32,
    y: i32,
}

impl fmt::String for Point {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Clone for Point {
    fn clone(&self) -> Point {
        Point { x:self.x, y:self.y }
    }
}

impl PartialEq for Point {
    fn eq(&self, other:&Point) -> bool {
        (self.x == other.x &&
         self.y == other.y)
    }
}

impl<H:hash::Hasher+hash::Writer> hash::Hash<H> for Point {
    fn hash(&self, state:&mut H) {
        self.x.hash(state);
    }
}

impl Eq for Point {
}

// Segment
struct Segment {
    name: char,
    locs: Vec<Point>,
}

impl Segment {
    fn new(name:char) -> Segment {
        Segment {
            name: name,
            locs: Vec::new(),
        }
    }

    fn add(&mut self, p:Point) {
        self.locs.push(p);
    }
}

// Board
struct Board {
    width: usize,
    height: usize,
    start: Point,
    goal: Point,
    segments: Vec<Segment>,
    loc2seg: HashMap<Point, usize>,
}

impl Board {
    fn new() -> Board {
        Board {
            width: 0,
            height: 0,
            start: Point { x: 0, y: 0 },
            goal: Point { x: 0, y: 0 },
            segments: Vec::new(),
            loc2seg: HashMap::new(),
        }
    }

    fn getseg(&self, x:i32, y:i32) -> Option<&usize> {
        let p = Point { x:x, y:y };
        self.loc2seg.get(&p)
    }
    
    fn show(&self) {
        for y in 0..self.height {
            let y = (self.height-y) as i32;
            let mut row:String = String::new();
            for x in 0..self.width {
                let x = x as i32;
                let p = Point { x:x, y:y };
                if (self.start == p) {
                    row.push('@');
                } else if (self.goal == p) {
                    row.push('*');
                } else {
                    match (self.getseg(x, y)) {
                        Some(i) => { row.push(self.segments[*i].name); }
                        None => { row.push('.'); }
                    }
                }
            }
            println!(" {}", row);
        }
    }
    
    fn load(&mut self, lines:Vec<String>) {
        let mut d:VecMap<usize> = VecMap::new();
        for (y,s) in lines.iter().enumerate() {
            let y = (lines.len()-y) as i32;
            let mut width = 0;
            for (x,c) in s.as_slice().chars().enumerate() {
                let x = x as i32;
                let p = Point { x:x, y:y };
                //println!("{}={}", p, c);
                if CharExt::is_whitespace(c) { continue; }
                if c == '@' {
                    self.start = p.clone();
                } else if c == '*' {
                    self.goal = p.clone();
                } else if CharExt::is_alphanumeric(c) {
                    let i = match d.get(&(c as usize)) {
                        Some(v) => { *v }
                        None => {
                            let n = self.segments.len();
                            self.segments.push(Segment::new(c));
                            n
                        }
                    };
                    d.insert(c as usize, i);
                    self.loc2seg.insert(p.clone(), i);
                    self.segments[i].add(p.clone());
                }
                width += 1;
            }
            self.width = width;
            self.height = self.height+1;
        }
    }
}

// Config
struct Config {
    board: Board,
    depths: Vec<usize>,
}

impl Config {
    fn new(board:Board, depths:Vec<usize>) -> Config {
        Config {
            board: board,
            depths: depths,
        }
    }

    fn show(self, loc:Point) {
        for y in 0..self.board.height {
            let y = (self.board.height-y) as i32;
            let mut row:String = String::new();
            for x in 0..self.board.width {
                let x = x as i32;
                let p = Point { x:x, y:y };
                if p == loc {
                    row.push('@');
                } else if p == self.board.goal {
                    row.push('*');
                } else {
                    match (self.board.getseg(x, y)) {
                        Some(i) => {
                            let c = char::from_u32(self.depths[*i] as u32);
                            row.push(c.expect("Invalid char"));
                        }
                        None => { row.push('.'); }
                    }
                }
            }
            println!(" {}", row);
        }
    }

}

fn main() {
    let args = os::args();
    let pathname = args[1].clone();
    let path = Path::new(pathname.into_bytes());
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => panic!("file error: {}", e),
    };
    let mut reader = BufferedReader::new(file);
    let mut board = Board::new();
    let mut lines = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(s) => { lines.push(s); }
            Err(e) => panic!("read error: {}", e),
        }
    }
    board.load(lines);
    board.show();
}
