use std::{
    collections::HashSet,
    fs::{self, File},
    io::{self, Read},
    ops::AddAssign,
    path::Path,
};

fn main() {
    let file_path = "../input.txt";
    let width = 7;
    let spawn_height = 4;
    let stop_after = 30;
    let left_edge_offset = 2;

    let path = Path::new(file_path);
    let file = File::open(path).unwrap();
    let jets = io::BufReader::new(file)
        .bytes()
        .map(|byte| byte.unwrap())
        .map(|byte| byte as char)
        .filter_map(|character| match character {
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        })
        .collect::<Vec<_>>();

    let rocks: Vec<_> = fs::read_to_string("../rocks.txt")
        .unwrap()
        .split("\r\n\r\n")
        .map(|string| {
            let mut shape = Shape::<i32>::default();
            let rows = string.split("\n").map(|slice| {
                if slice.len() > width {
                    panic!("Invalid shape data too wide");
                }
                slice
                    .chars()
                    .enumerate()
                    .filter_map(|(index, character)| match character {
                        '#' => Some(index),
                        _ => None,
                    })
            });
            let offset = width as i32;

            for row in rows {
                shape.step(offset);
                shape.coordinates.extend(row.map(|index| index as i32));
            }

            let left_edge = shape
                .coordinates
                .iter()
                .map(|coordinate| coordinate % offset)
                .min()
                .unwrap();
            (shape, left_edge)
        })
        .collect();
    println!("{:?}", rocks);

    let mut jets = jets.iter().cycle();
    let mut rock_formation = HashSet::new();
    let mut top_row = 0;
    let row_offset = width as i32;

    for (mut rock, left_edge) in rocks.iter().map(|shape| shape.clone()).cycle() {
        let spawn_row = top_row + spawn_height;
        rock.step(spawn_row * row_offset + left_edge); // adjust start position of rock
        loop {
            let mut clone = rock.clone();
            let side_push = match jets.next().unwrap() {
                Direction::Left
                    if clone
                        .coordinates
                        .iter()
                        .all(|coordinate| coordinate % row_offset > 0) =>
                {
                    -1
                }
                Direction::Right
                    if clone
                        .coordinates
                        .iter()
                        .all(|coordinate| coordinate % row_offset < row_offset - 1) =>
                {
                    1
                }

                _ => 0,
            };

            clone.step(side_push + row_offset); // get pushed and move down
            if clone
                .coordinates
                .iter()
                .any(|coordinate| *coordinate < 0 || rock_formation.contains(coordinate))
            {
                //collision
                top_row = top_row.max(rock.coordinates.iter().max().unwrap() / row_offset);
                rock_formation.extend(rock.coordinates);
                break;
            }
            rock = clone;
        }
    }

    let display_rows = 20..=0;
    for y in display_rows{
        for x in 0..row_offset{
            let coordinate = y*row_offset + x;
            let character = match rock_formation.contains(&coordinate){
                true => '#',
                false => '.',
            };
            print!("{}", character);
        }
        println!()
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Right,
    Left,
}

#[derive(Default, Debug, Clone)]
struct Shape<T> {
    coordinates: Vec<T>,
}

impl<T> Shape<T> {
    fn step<Offset>(&mut self, offset: Offset)
    where
        Offset: Copy,
        T: AddAssign<Offset>,
    {
        for coordinate in self.coordinates.iter_mut() {
            coordinate.add_assign(offset);
        }
    }
}
