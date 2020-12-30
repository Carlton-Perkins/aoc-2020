/*

*/

use std::ops::Add;
use clap::{App, Arg};
use std::{collections::HashSet, fs};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Add for Position {
    type Output = Position;
    fn add(self, other: Position) -> <Self as std::ops::Add<Position>>::Output {
        Position{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

type Grid = HashSet<Position>;

fn main() {
    let matches = App::new("day1")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true),
        )
        .get_matches();

    let file = matches.value_of("file").unwrap();
    let input = fs::read_to_string(file).unwrap();

    let lines: Vec<Vec<char>> = input.split("\n").filter(|s| !s.is_empty()).map(|x| x.chars().collect()).collect();


    let mut grid = Grid::new();
    let mut y = 0;
    for ypos in lines {
        let mut x = 0;
        for xpos in  ypos {
            if xpos == '#' {
                grid.insert(Position{ x, y, z: 0});
            }
            x += 1;
        }
        y += 1;
    }


    // println!("Grid: {:?}", grid);

    let iters = 6;
    for _ in 0..iters {
        let (min,max) = grid_bounds(&grid);
        // println!("GridBounds: {:?} {:?}", min, max);

        let mut new_grid = Grid::new();
        let start = Position{x: min.x - 1, y: min.y - 1, z: min.z - 1};
        for x in start.x..=max.x+1 {
            for y in start.y..=max.y+1 {
                for z in start.z..=max.z+1 {
                    let pos = Position{x,y,z};
                    // println!("{},{},{} -> {}", x,y,z, check_adj(&grid, &Position{x,y,z}))
                    let adj_count = check_adj(&grid, &pos);
                    if check(&grid, &pos) {
                        if adj_count == 2 || adj_count == 3 {
                            new_grid.insert(pos);
                        }
                    } else {
                        if adj_count == 3 {
                            new_grid.insert(pos);
                        }
                    }
                }
            }
        }
        grid = new_grid
    }
    println!("Nodes: {}", grid.len());

}


fn grid_bounds(grid: &Grid) -> (Position, Position) {
    let mut min: Position = grid.iter().take(1).map(|x| x.clone()).nth(0).unwrap();
    let mut max: Position = grid.iter().take(1).map(|x| x.clone()).nth(0).unwrap();

    for pos in grid {
        min = Position{
            x: (min.x.min(pos.x)),
            y: (min.y.min(pos.y)),
            z: (min.z.min(pos.z)),
        };
        max = Position{
            x: (max.x.max(pos.x)),
            y: (max.y.max(pos.y)),
            z: (max.z.max(pos.z)),
        };
    }

    (min, max)
}

fn check(grid: &Grid, pos: &Position) -> bool {
    grid.contains(pos)
}

fn check_adj(grid: &Grid, pos: &Position) -> usize {
    // Almost certainly a better way to do this
    let adj = vec![
        Position{x: (-1), y: (-1), z: (-1)},
        Position{x: (-1), y: (-1), z: (0)},
        Position{x: (-1), y: (-1), z: (1)},
        Position{x: (-1), y: (0), z: (-1)},
        Position{x: (-1), y: (0), z: (0)},
        Position{x: (-1), y: (0), z: (1)},
        Position{x: (-1), y: (1), z: (-1)},
        Position{x: (-1), y: (1), z: (0)},
        Position{x: (-1), y: (1), z: (1)},

        Position{x: (0), y: (-1), z: (-1)},
        Position{x: (0), y: (-1), z: (0)},
        Position{x: (0), y: (-1), z: (1)},
        Position{x: (0), y: (0), z: (-1)},
        // Position{x: (0), y: (0), z: (0)}, Don't need this one
        Position{x: (0), y: (0), z: (1)},
        Position{x: (0), y: (1), z: (-1)},
        Position{x: (0), y: (1), z: (0)},
        Position{x: (0), y: (1), z: (1)},

        Position{x: (1), y: (-1), z: (-1)},
        Position{x: (1), y: (-1), z: (0)},
        Position{x: (1), y: (-1), z: (1)},
        Position{x: (1), y: (0), z: (-1)},
        Position{x: (1), y: (0), z: (0)},
        Position{x: (1), y: (0), z: (1)},
        Position{x: (1), y: (1), z: (-1)},
        Position{x: (1), y: (1), z: (0)},
        Position{x: (1), y: (1), z: (1)},
    ];
    adj.into_iter().filter(|x| check(grid, &(*x + *pos))).count()
}
