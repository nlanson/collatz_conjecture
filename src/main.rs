/*
    Collatz Conjecture

    This program is a tool used to run through digits in the Collatz conjecture.
    This is an unsolved mathematical problem for when a positive number is put into a 
    sequence and applied the rules 3x+1 if odd or divided by 2 when even that eventually 
    the sequence results in a loop of the integers 4, 2, 1... 

    This program aims to visualise this conjecture on a graph.
*/

use text_io::read;
use plotters::prelude::*;

struct Sequence {
    start: u64,
    coords: Vec<Coord>
}

impl Sequence {
    pub fn new(start: u64) -> Self {
        Self {
            start,
            coords: Vec::new()
        }
    }

    pub fn odd(&mut self, x: u64, y: u64) {
        let rule_applied: u64 = (3*y)+1;
        let coord: Coord = Coord::new(x, rule_applied);
        self.coords.push(coord);
    }
    
    pub fn even(&mut self, x: u64, y: u64) {
        let rule_applied: u64 = y/2;
        let coord: Coord = Coord::new(x, rule_applied);
        self.coords.push(coord);
    }
}

struct Coord {
    x: u64,
    y: u64
}

impl Coord {
    pub fn new(x: u64, y: u64) -> Self {
        Coord { x, y }
    }

    pub fn to_tuple_vec(coords: &Vec<Self>) -> Vec<(u64, u64)> {
        let mut vec = Vec::new();
        for i in 0..coords.len() {
            vec.push((coords[i].x as u64, coords[i].y as u64))
        }

        return vec;
    }

    pub fn find_highest_in_tuple_vec(coords: &Vec<(u64, u64)>) -> u64 {
        let mut max: u64 = 0;

        for i in 0..coords.len() {
            if coords[i].1 > max {
                max = coords[i].1
            }
        }

        max
    }
}

fn main() {
    println!("Enter starting number:");
    let start_int: u64 = read!();

    if start_int <= 0 {
        println!("Invalid number");
        return;
    }

    let mut sequence: Sequence = Sequence::new(start_int);
    sequence.coords.push(Coord::new(0, start_int));

    let mut count: u64 = 1;
    loop {
        if sequence.coords[(count-1) as usize].y % 2 == 0 {
            sequence.even(count, sequence.coords[(count-1) as usize].y);
        } else {
            sequence.odd(count, sequence.coords[(count-1) as usize].y);
        }

        if sequence.coords[count as usize].y == 1 {
            break;
        }

        count += 1;
    }

    println!("Reached the 4, 2, 1 loop in {} loops", count);
    match plot(sequence) { _=>{}};
    return;
}

//Takes in a a sequence and outputs a plot image.
//Currently only takes in one sequence. Could later make it take in a vector of 
//sequences to compare conjectures.
fn plot(sequence: Sequence) -> Result<(), Box<dyn std::error::Error>> {
    let coords: Vec<(u64, u64)> = Coord::to_tuple_vec(&sequence.coords);
    let start_int: u64 = sequence.start;
    
    //Setup plot
    let root = BitMapBackend::new("./out.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(10, 10, 10, 10);
    let mut chart = ChartBuilder::on(&root)
        .caption(format!("Collatz conjecture (From {})", start_int), ("sans-serif", 40).into_font())
        .x_label_area_size(20)
        .y_label_area_size(40)
        .build_cartesian_2d(0u64..coords.len() as u64, 0u64..Coord::find_highest_in_tuple_vec(&coords))?;

    // Draw mesh
    chart
        .configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;

    // Draw line
    chart.draw_series(LineSeries::new(
        coords.clone(),
        &RED,
    ))?;

    Ok(())
}
