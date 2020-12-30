/*
--- Part Two ---
For some reason, your simulated results don't match what the experimental energy source engineers expected. Apparently, the pocket dimension actually has four spatial dimensions, not three.

The pocket dimension contains an infinite 4-dimensional grid. At every integer 4-dimensional coordinate (x,y,z,w), there exists a single cube (really, a hypercube) which is still either active or inactive.

Each cube only ever considers its neighbors: any of the 80 other cubes where any of their coordinates differ by at most 1. For example, given the cube at x=1,y=2,z=3,w=4, its neighbors include the cube at x=2,y=2,z=3,w=3, the cube at x=0,y=2,z=3,w=4, and so on.

The initial state of the pocket dimension still consists of a small flat region of cubes. Furthermore, the same rules for cycle updating still apply: during each cycle, consider the number of active neighbors of each cube.

For example, consider the same initial state as in the example above. Even though the pocket dimension is 4-dimensional, this initial state represents a small 2-dimensional slice of it. (In particular, this initial state defines a 3x3x1x1 region of the 4-dimensional space.)

Simulating a few cycles from this initial state produces the following configurations, where the result of each cycle is shown layer-by-layer at each given z and w coordinate:

Before any cycles:

z=0, w=0
.#.
..#
###


After 1 cycle:

z=-1, w=-1
#..
..#
.#.

z=0, w=-1
#..
..#
.#.

z=1, w=-1
#..
..#
.#.

z=-1, w=0
#..
..#
.#.

z=0, w=0
#.#
.##
.#.

z=1, w=0
#..
..#
.#.

z=-1, w=1
#..
..#
.#.

z=0, w=1
#..
..#
.#.

z=1, w=1
#..
..#
.#.


After 2 cycles:

z=-2, w=-2
.....
.....
..#..
.....
.....

z=-1, w=-2
.....
.....
.....
.....
.....

z=0, w=-2
###..
##.##
#...#
.#..#
.###.

z=1, w=-2
.....
.....
.....
.....
.....

z=2, w=-2
.....
.....
..#..
.....
.....

z=-2, w=-1
.....
.....
.....
.....
.....

z=-1, w=-1
.....
.....
.....
.....
.....

z=0, w=-1
.....
.....
.....
.....
.....

z=1, w=-1
.....
.....
.....
.....
.....

z=2, w=-1
.....
.....
.....
.....
.....

z=-2, w=0
###..
##.##
#...#
.#..#
.###.

z=-1, w=0
.....
.....
.....
.....
.....

z=0, w=0
.....
.....
.....
.....
.....

z=1, w=0
.....
.....
.....
.....
.....

z=2, w=0
###..
##.##
#...#
.#..#
.###.

z=-2, w=1
.....
.....
.....
.....
.....

z=-1, w=1
.....
.....
.....
.....
.....

z=0, w=1
.....
.....
.....
.....
.....

z=1, w=1
.....
.....
.....
.....
.....

z=2, w=1
.....
.....
.....
.....
.....

z=-2, w=2
.....
.....
..#..
.....
.....

z=-1, w=2
.....
.....
.....
.....
.....

z=0, w=2
###..
##.##
#...#
.#..#
.###.

z=1, w=2
.....
.....
.....
.....
.....

z=2, w=2
.....
.....
..#..
.....
.....
After the full six-cycle boot process completes, 848 cubes are left in the active state.

Starting with your given initial configuration, simulate six cycles in a 4-dimensional space. How many cubes are left in the active state after the sixth cycle?
*/

use std::ops::Add;
use clap::{App, Arg};
use std::{collections::HashSet, fs};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Add for Position {
    type Output = Position;
    fn add(self, other: Position) -> <Self as std::ops::Add<Position>>::Output {
        Position{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
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
                grid.insert(Position{ x, y, z: 0, w: 0});
            }
            x += 1;
        }
        y += 1;
    }

    // Calculate adj matrix
    let mut adj = vec![];
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                for w in -1..=1 {
                    adj.push(Position{x,y,z,w});
                }
            }
        }
    }
    let idx = adj.iter().position(|x| *x == Position{ x: (0), y: (0), z: (0), w: (0)}).unwrap();
    adj.remove(idx);

    // println!("Grid: {:?}", grid);

    let iters = 6;
    for _ in 0..iters {
        let (min,max) = grid_bounds(&grid);
        // println!("GridBounds: {:?} {:?}", min, max);

        let mut new_grid = Grid::new();
        let start = Position{x: min.x - 1, y: min.y - 1, z: min.z - 1, w: min.w -1};
        for x in start.x..=max.x+1 {
            for y in start.y..=max.y+1 {
                for z in start.z..=max.z+1 {
                    for w in start.w..=max.w+1 {
                        let pos = Position{x,y,z,w};
                        // println!("{},{},{} -> {}", x,y,z, check_adj(&grid, &Position{x,y,z}))
                        let adj_count = check_adj(&grid, &pos, &adj);
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
            w: (min.w.min(pos.w)),
        };
        max = Position{
            x: (max.x.max(pos.x)),
            y: (max.y.max(pos.y)),
            z: (max.z.max(pos.z)),
            w: (max.w.max(pos.w)),
        };
    }

    (min, max)
}

fn check(grid: &Grid, pos: &Position) -> bool {
    grid.contains(pos)
}

fn check_adj(grid: &Grid, pos: &Position, adj: &Vec<Position>) -> usize {
    adj.into_iter().filter(|x| check(grid, &(**x + *pos))).count()
}
