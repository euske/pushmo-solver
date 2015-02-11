// pushmo.rs

use std::io::{File, BufferedReader};
use std::collections::{VecMap, HashMap, HashSet};
use std::collections;
use std::char;
use std::iter;
use std::usize;
use std::hash;
use std::fmt;
use std::os;

const INF:usize = usize::MAX;

// misc. functions
fn max(x:usize, y:usize) -> usize {
    if (x < y) { y } else { x }
}
fn min(a:&[usize]) -> usize {
    let mut m = usize::MAX;
    for v in a.iter() {
        if *v < m { m = *v; }
    }
    return m;
}


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

fn fmtpts(pts:&HashSet<Point>) -> String {
    let mut s = String::new();
    let mut n = 0;
    s.push_str("[");
    for i in pts.iter() {
        if 0 < n { s.push_str(", "); }
        s.push_str(format!("{}", *i).as_slice());
        n += 1;
    }
    s.push_str("]");
    return s;
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
                if self.start == p {
                    row.push('@');
                } else if self.goal == p {
                    row.push('*');
                } else {
                    match self.getseg(x, y) {
                        Some(&i) => { row.push(self.segments[i].name); }
                        None => { row.push('.'); }
                    }
                }
            }
            println!(" {}", row);
        }
    }
    
    fn load(&mut self, lines:&Vec<String>) {
        let mut d:VecMap<usize> = VecMap::new();
        for (y,s) in lines.iter().enumerate() {
            let y = (lines.len()-y) as i32;
            let mut width = 0;
            for (x,c) in s.as_slice().chars().enumerate() {
                let x = x as i32;
                let p = Point { x:x, y:y };
                //println!("{}={}", p, c);
                if char::CharExt::is_whitespace(c) { continue; }
                if c == '@' {
                    self.start = p.clone();
                } else if c == '*' {
                    self.goal = p.clone();
                } else if char::CharExt::is_alphanumeric(c) {
                    let i = match d.get(&(c as usize)) {
                        Some(&v) => { v }
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
struct Config<'a> {
    board: &'a Board,
    depths: Vec<usize>,
}

impl<'a> PartialEq for Config<'a> {
    fn eq(&self, other:&Config) -> bool {
        self.depths == other.depths
    }
}

impl<'a, H:hash::Hasher+hash::Writer> hash::Hash<H> for Config<'a> {
    fn hash(&self, state:&mut H) {
        self.depths.hash(state);
    }
}

impl<'a> Eq for Config<'a> {
}

struct SRange {
    seg: usize,
    z0: usize,
    z1: usize,
}

impl <'a>Config<'a> {
    fn init(board:&Board) -> Config {
        let mut depths = Vec::new();
        for i in 0..(board.segments.len()) {
            depths.push(0);
        }
        return Config::new(board, depths);
    }
    
    fn new(board:&Board, depths:Vec<usize>) -> Config {
        Config {
            board: board,
            depths: depths,
        }
    }

    fn show(&self, loc:&Point) {
        for y in 0..self.board.height {
            let y = (self.board.height-y) as i32;
            let mut row:String = String::new();
            for x in 0..self.board.width {
                let x = x as i32;
                let p = Point { x:x, y:y };
                if p == *loc {
                    row.push('@');
                } else if p == self.board.goal {
                    row.push('*');
                } else {
                    match self.board.getseg(x, y) {
                        Some(&i) => {
                            let c = char::from_u32(self.depths[i] as u32);
                            row.push(c.expect("Invalid char"));
                        }
                        None => { row.push('.'); }
                    }
                }
            }
            println!(" {}", row);
        }
    }

    fn getdepth(&self, x:i32, y:i32) -> usize {
        if y < 0 {
            INF
        } else {
            match self.board.getseg(x, y) {
                Some(&i) => { self.depths[i] }
                None => { 0 }
            }
        }
    }

    fn getsegs(&self, loc:&Point) -> Vec<SRange> {
        let x = loc.x;
        let y = loc.y;
        let z = self.getdepth(x, y);
        // platform block.
        let sp = self.board.getseg(x, y-1);
        let zp = self.getdepth(x, y-1);
        let mut r = Vec::new();
        // front block.
        let sf = self.board.getseg(x, y);
        match sf {
            Some(&i) => {
                if sf != sp && z < zp-1 {
                    if 0 < y {
                        r.push(SRange { seg:i, z0:0, z1:zp-1 });
                    } else {
                        r.push(SRange { seg:i, z0:0, z1:-1 });
                    }
                }
            }
            None => {}
        }
        // side block. (left)
        let sl = self.board.getseg(x-1, y);
        let zl = self.getdepth(x-1, y);
        match sl {
            Some(&i) => {
                if sl != sp && z < zl {
                    r.push(SRange { seg:i, z0:z+1, z1:-1 });
                }
            }
            None => {}
        }
        // side block. (right)
        let sr = self.board.getseg(x+1, y);
        let zr = self.getdepth(x+1, y);
        match sr {
            Some(&i) => {
                if sr != sp && z < zr {
                    r.push(SRange { seg:i, z0:z+1, z1:-1 });
                }
            }
            None => {}
        }
        return r;
    }

    fn getlocs(&self, locs:&mut HashSet<Point>, x0:i32, y0:i32) {
        if x0 < 0 || self.board.width < x0 as usize {
            return;
        }
        let p = Point { x:x0, y:y0 };
        if locs.contains(&p) {
            return;
        }
        locs.insert(p);
        let z0 = self.getdepth(x0, y0);
        let zp = self.getdepth(x0, y0-1);
        // check jumping.
        if self.getdepth(x0, y0+1) < zp {
            for dx in [-1,0,1].iter() {
                let x1 = x0+*dx;
                let z = self.getdepth(x1, y0+1);
                if zp <= z { continue; }
                if max(z,z0-1) < self.getdepth(x1, y0) {
                    //print ' jump1', (x0,y0), (x1, y0+1);
                    self.getlocs(locs, x1, y0+1);
                    continue
                }
            }
        }
        for dx in [-1,1].iter() {
            let x1 = x0+*dx;
            if self.getdepth(x1, y0+1) < zp {
                let x2 = x1+*dx;
                let z1 = self.getdepth(x2, y0+1);
                if zp <= z1 { continue; }
                if max(z1,z0) < self.getdepth(x2, y0) {
                    //print ' jump2', (x0,y0), (x2, y0+1);
                    self.getlocs(locs, x2, y0+1);
                    continue;
                }
                let z2 = self.getdepth(x2, y0);
                if zp <= z2 { continue; }
                if max(z2,z0) < self.getdepth(x2, y0-1) {
                    //print ' jump3', (x0,y0), (x2, y0);
                    self.getlocs(locs, x2, y0);
                    continue;
                }
            }
        }
        // check walking/falling.
        for dx in [-1,1].iter() {
            let x1 = x0+*dx;
            let z = self.getdepth(x1, y0);
            if zp <= z { continue; }
            for dy in iter::range_inclusive(0, y0) {
                let y1 = y0-dy;
                if max(z,z0) < self.getdepth(x1, y1-1) {
                    //print ' walk/fall', (x0,y0), (x1, y1);
                    self.getlocs(locs, x1, y1);
                    break;
                }
            }
        }
        // check falling.
        for dx in [-1,0,1].iter() {
            let x1 = x0+*dx;
            let z = self.getdepth(x1, y0);
            let mut z = max(z, zp);
            for dy in iter::range_inclusive(0, y0) {
                let y1 = y0-dy;
                let z1 = self.getdepth(x1, y1-1);
                if z < z1 {
                    //print ' fall', (x0,y0), (x1, y1), z,z1;
                    self.getlocs(locs, x1, y1);
                    break;
                }
                z = max(z,z1);
            }
        }
        return;
    }
    
}

struct State<'a> {
    n: usize,
    prev: Option<Box<State<'a>>>,
    config: Config<'a>,
    loc: Point,
}

fn solve_pushmo(board:&Board, verbose:bool, max_depth:usize) -> Vec<State> {
    let mut config = Config::init(board);
    let mut queue = Vec::new();
    queue.push(State { n:0, prev:None, config:config, loc:board.start });
    let mut states:HashMap<&Config, Vec<HashSet<Point>>> = HashMap::new();
    let mut solution = None;
    while (0 < queue.len()) {
        queue.sort_by(|a, b| a.n.cmp(&b.n));
        let prev = &queue.remove(0);
        let n = prev.n+1;
        let mut locsets = match states.get(&prev.config) {
            Some(sets) => { sets }
            None => {
                let empty = Vec::new();
                states.insert(&prev.config, empty);
                &empty
            }
        };
        let mut found = false;
        for locs in locsets.iter() {
            if locs.contains(&prev.loc) {
                found = true;
                break;
            }
        }
        if found { continue; }
        config = prev.config;
        if verbose {
            print!("-- Move {} --", n);
            config.show(&prev.loc)
        }
        let mut newlocs = HashSet::new();
        config.getlocs(&mut newlocs, prev.loc.x, prev.loc.y);
        if verbose {
            print!(" Possible locations: {}", fmtpts(&newlocs));
        }
        if newlocs.contains(&board.goal) {
            solution = Some(State {
                n:n,
                prev:Some(Box::new(*prev)),
                config:config,
                loc:board.goal
            });
            break;
        }
        for loc in newlocs.iter() {
            for srange in config.getsegs(loc).iter() {
                let i = srange.seg;
                let z1 = if (0 <= srange.z1) { srange.z1 } else { max_depth };
                let depths = config.depths.as_slice();
                for z in srange.z0..z1+1 {
                    let mut d = Vec::new();
                    d.push_all(&depths[..i]);
                    d.push(z);
                    d.push_all(&depths[i+1..]);
                    if 2 <= min(d.as_slice()) {
                        // all blocks pulled out by 2 - pointless.
                        continue;
                    }
                    let next = Config { board:board, depths:d };
                    if states.contains_key(&next) { continue; }
                    queue.push(State {
                        n:n,
                        prev:Some(Box::new(*prev)),
                        config:next,
                        loc:*loc
                    });
                }
            }
        }
        locsets.push(newlocs);
    }
    let mut r = Vec::new();
    while solution.is_some() {
        match solution {
            Some(state) => {
                r.insert(0, state);
                match state.prev {
                    Some(prev) => {
                        solution = Some(*prev);
                    }
                    _ => {
                        solution = None;
                    }
                }
            }
            _ => {}
        }
    }
    return r;
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
    board.load(&lines);
    board.show();
}
