use std::{
    fs,
    ops::{Index, IndexMut},
};

#[derive(Debug)]
struct Grid<T> {
    size: usize,
    arr: Vec<T>,
}

impl<T: Default> Grid<T> {
    pub fn new(size: usize) -> Self {
        let mut arr = Vec::new();
        arr.resize_with(size * size, Default::default);
        // println!("{:?}", arr.len());
        Grid::<T> { size, arr }
    }
}

impl Grid<(u8, bool)> {
    pub fn count_trees(&self) -> usize {
        self.arr.iter().filter(|x| x.1).count()
    }
}
impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.arr[self.size * index.0 + index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.arr[self.size * index.0 + index.1]
    }
}

impl From<Vec<&str>> for Grid<(u8, bool)> {
    fn from(s: Vec<&str>) -> Self {
        let mut grid = Grid::new(s.len());
        for (x, line) in s.iter().enumerate() {
            for (y, ch) in line.chars().enumerate() {
                let val: u8 = ch.to_digit(10).expect("Bad input") as u8;
                grid[(x, y)] = (val, false);
            }
        }
        grid
    }
}

fn main() {
    let input = fs::read_to_string("./data/in").expect("Something went wrong");
    let lines = input.lines().collect::<Vec<_>>();
    let mut visible: Grid<(u8, bool)> = lines.into();

    for x in 0..visible.size {
        let mut last_visible_height = visible[(x, 0)].0;
        visible[(x, 0)].1 = true;
        for y in 1..visible.size {
            if last_visible_height < visible[(x, y)].0 {
                last_visible_height = visible[(x, y)].0;
                visible[(x, y)].1 = true;
            }
        }
    }

    for x in (0..visible.size).rev() {
        let y = visible.size - 1;
        let mut last_visible_height = visible[(x, y)].0;
        visible[(x, y)].1 = true;
        for y in (1..visible.size).rev() {
            if last_visible_height < visible[(x, y)].0 {
                last_visible_height = visible[(x, y)].0;
                visible[(x, y)].1 = true;
            }
        }
    }

    for y in 0..visible.size {
        let mut last_visible_height = visible[(0, y)].0;
        visible[(0, y)].1 = true;
        for x in 1..visible.size {
            if last_visible_height < visible[(x, y)].0 {
                last_visible_height = visible[(x, y)].0;
                visible[(x, y)].1 = true;
            }
        }
    }

    for y in (0..visible.size).rev() {
        let x = visible.size - 1;
        let mut last_visible_height = visible[(x, y)].0;
        visible[(x, y)].1 = true;
        for x in (1..visible.size).rev() {
            if last_visible_height < visible[(x, y)].0 {
                last_visible_height = visible[(x, y)].0;
                visible[(x, y)].1 = true;
            }
        }
    }
    println!("{:?}", visible);
    println!("{:?}", visible.count_trees());
}
