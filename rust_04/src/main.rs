use clap::Parser;
use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{self, Read, Write};
use std::thread;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(required_unless_present = "generate")]
    input: Option<String>,

    #[arg(short, long)]
    generate: Option<String>,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(short, long)]
    visualize: bool,

    #[arg(long)]
    both: bool,

    #[arg(long)]
    animate: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, Eq, PartialEq)]
struct Node {
    cost: u32,
    pos: Point,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Vec<u8>>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut cells = vec![vec![0; width]; height];

        for (y, row) in cells.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                if x == 0 && y == 0 {
                    *cell = 0x00;
                } else if x == width - 1 && y == height - 1 {
                    *cell = 0xFF;
                } else {
                    *cell = rng.gen();
                }
            }
        }
        Grid {
            width,
            height,
            cells,
        }
    }

    fn from_file(path: &str) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let values: Vec<u8> = content
            .split_whitespace()
            .map(|s| u8::from_str_radix(s, 16).unwrap_or(0))
            .collect();

        let lines: Vec<&str> = content.lines().collect();
        let height = lines.len();
        let width = if height > 0 { values.len() / height } else { 0 };

        let mut cells = vec![vec![0; width]; height];
        for (i, &val) in values.iter().enumerate() {
            cells[i / width][i % width] = val;
        }

        Ok(Grid {
            width,
            height,
            cells,
        })
    }

    fn save(&self, path: &str) -> io::Result<()> {
        let mut file = File::create(path)?;
        for row in &self.cells {
            let line: Vec<String> = row.iter().map(|b| format!("{:02X}", b)).collect();
            writeln!(file, "{}", line.join(" "))?;
        }
        Ok(())
    }

    fn get_color(val: u8) -> Color {
        match val {
            0..=42 => Color::Red,
            43..=85 => Color::Yellow,
            86..=127 => Color::Green,
            128..=170 => Color::Cyan,
            171..=212 => Color::Blue,
            _ => Color::Magenta,
        }
    }

    fn display(&self, path: Option<&Vec<Point>>, path_color: Color) {
        let path_set: Vec<Point> = path.cloned().unwrap_or_default();

        println!("{}", "HEXADECIMAL GRID:".bold());
        println!("{}", "-".repeat(self.width * 3));

        for y in 0..self.height {
            for x in 0..self.width {
                let val = self.cells[y][x];
                let is_in_path = path_set.contains(&Point { x, y });

                let text = format!("{:02X}", val);

                if is_in_path {
                    print!("{} ", text.color(path_color).bold().on_black());
                } else {
                    print!("{} ", text.color(Self::get_color(val)));
                }
            }
            println!();
        }
        println!();
    }
}

fn dijkstra(
    grid: &Grid,
    start: Point,
    end: Point,
    animate: bool,
    maximize: bool,
) -> Option<(u32, Vec<Point>)> {
    let mut dists: HashMap<Point, u32> = HashMap::new();
    let mut came_from: HashMap<Point, Point> = HashMap::new();
    let mut pq = BinaryHeap::new();

    dists.insert(start, 0);
    pq.push(Node {
        cost: 0,
        pos: start,
    });

    let mut step_count = 0;

    while let Some(Node { cost, pos }) = pq.pop() {
        if animate && step_count % 5 == 0 {
            print!("\rSearching... Visited nodes: {}", step_count);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(10));
        }
        step_count += 1;

        if pos == end {
            if animate {
                println!("\nPath found!");
            }
            let mut path = Vec::new();
            let mut current = end;
            path.push(current);
            while let Some(&prev) = came_from.get(&current) {
                path.push(prev);
                current = prev;
            }
            path.reverse();

            let total_real_cost: u32 = path.iter().map(|p| grid.cells[p.y][p.x] as u32).sum();

            let final_cost = total_real_cost - grid.cells[start.y][start.x] as u32;
            return Some((final_cost, path));
        }

        if let Some(&d) = dists.get(&pos) {
            if cost > d {
                continue;
            }
        }

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        for (dx, dy) in directions.iter() {
            let new_x = pos.x as isize + dx;
            let new_y = pos.y as isize + dy;

            if new_x >= 0
                && new_x < grid.width as isize
                && new_y >= 0
                && new_y < grid.height as isize
            {
                let neighbor = Point {
                    x: new_x as usize,
                    y: new_y as usize,
                };
                let cell_val = grid.cells[neighbor.y][neighbor.x] as u32;

                let weight = if maximize { 2550 - cell_val } else { cell_val };

                let new_cost = cost + weight;

                if new_cost < *dists.get(&neighbor).unwrap_or(&u32::MAX) {
                    dists.insert(neighbor, new_cost);
                    came_from.insert(neighbor, pos);
                    pq.push(Node {
                        cost: new_cost,
                        pos: neighbor,
                    });
                }
            }
        }
    }

    None
}

fn main() {
    let args = Args::parse();

    let grid = if let Some(gen_arg) = args.generate {
        let parts: Vec<&str> = gen_arg.split('x').collect();
        let w: usize = parts[0].parse().expect("Largeur invalide");
        let h: usize = parts
            .get(1)
            .unwrap_or(&parts[0])
            .parse()
            .expect("Hauteur invalide");

        println!("Generating {}x{} hexadecimal grid...", w, h);
        let g = Grid::new(w, h);

        if let Some(out_path) = args.output {
            g.save(&out_path).expect("Erreur sauvegarde");
            println!("Map saved to: {}", out_path);
        }
        g
    } else {
        let path = args.input.expect("Input file required unless generating");
        println!("Analyzing map: {}", path);
        Grid::from_file(&path).expect("Impossible de lire le fichier map")
    };

    println!("Grid size: {}x{}", grid.width, grid.height);
    if args.visualize {
        grid.display(None, Color::White);
    }

    let start = Point { x: 0, y: 0 };
    let end = Point {
        x: grid.width - 1,
        y: grid.height - 1,
    };
    println!("\n{}", "MINIMUM COST PATH:".underline());
    if let Some((cost, path)) = dijkstra(&grid, start, end, args.animate, false) {
        grid.display(Some(&path), Color::White); // Blanc pour le chemin min
        println!("Total cost: 0x{:X} ({} decimal)", cost, cost);
        println!("Path length: {} steps", path.len());
        println!("Step-by-step costs:");
        let mut accum = 0;
        for (i, p) in path.iter().enumerate() {
            if i == 0 {
                println!("Start 0x00 ({},{})", p.x, p.y);
            } else {
                let val = grid.cells[p.y][p.x] as u32;
                accum += val;
                println!(" -> 0x{:02X} ({},{}) +{}", val, p.x, p.y, accum);
            }
            if i > 5 && path.len() > 10 {
                println!(" ... ({} steps omitted)", path.len() - 7);
                break;
            }
        }
    } else {
        println!("No path found!");
    }
    if args.both {
        println!("\n{}", "MAXIMUM COST PATH:".underline());
        if let Some((cost, path)) = dijkstra(&grid, start, end, args.animate, true) {
            grid.display(Some(&path), Color::Red);
            println!("Total cost: 0x{:X} ({} decimal)", cost, cost);
            println!("Path length: {} steps", path.len());
        }
    }
}
