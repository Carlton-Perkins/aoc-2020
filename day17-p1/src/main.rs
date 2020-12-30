/*
--- Day 17: Conway Cubes ---
As your flight slowly drifts through the sky, the Elves at the Mythical Information Bureau at the North Pole contact you. They'd like some help debugging a malfunctioning experimental energy source aboard one of their super-secret imaging satellites.

The experimental energy source is based on cutting-edge technology: a set of Conway Cubes contained in a pocket dimension! When you hear it's having problems, you can't help but agree to take a look.

The pocket dimension contains an infinite 3-dimensional grid. At every integer 3-dimensional coordinate (x,y,z), there exists a single cube which is either active or inactive.

In the initial state of the pocket dimension, almost all cubes start inactive. The only exception to this is a small flat region of cubes (your puzzle input); the cubes in this region start in the specified active (#) or inactive (.) state.

The energy source then proceeds to boot up by executing six cycles.

Each cube only ever considers its neighbors: any of the 26 other cubes where any of their coordinates differ by at most 1. For example, given the cube at x=1,y=2,z=3, its neighbors include the cube at x=2,y=2,z=2, the cube at x=0,y=2,z=3, and so on.

During a cycle, all cubes simultaneously change their state according to the following rules:

If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.
The engineers responsible for this experimental energy source would like you to simulate the pocket dimension and determine what the configuration of cubes should be at the end of the six-cycle boot process.

For example, consider the following initial state:

.#.
..#
###
Even though the pocket dimension is 3-dimensional, this initial state represents a small 2-dimensional slice of it. (In particular, this initial state defines a 3x3x1 region of the 3-dimensional space.)

Simulating a few cycles from this initial state produces the following configurations, where the result of each cycle is shown layer-by-layer at each given z coordinate (and the frame of view follows the active cells in each cycle):

Before any cycles:

z=0
.#.
..#
###


After 1 cycle:

z=-1
#..
..#
.#.

z=0
#.#
.##
.#.

z=1
#..
..#
.#.


After 2 cycles:

z=-2
.....
.....
..#..
.....
.....

z=-1
..#..
.#..#
....#
.#...
.....

z=0
##...
##...
#....
....#
.###.

z=1
..#..
.#..#
....#
.#...
.....

z=2
.....
.....
..#..
.....
.....


After 3 cycles:

z=-2
.......
.......
..##...
..###..
.......
.......
.......

z=-1
..#....
...#...
#......
.....##
.#...#.
..#.#..
...#...

z=0
...#...
.......
#......
.......
.....##
.##.#..
...#...

z=1
..#....
...#...
#......
.....##
.#...#.
..#.#..
...#...

z=2
.......
.......
..##...
..###..
.......
.......
.......
After the full six-cycle boot process completes, 112 cubes are left in the active state.

Starting with your given initial configuration, simulate six cycles. How many cubes are left in the active state after the sixth cycle?
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
