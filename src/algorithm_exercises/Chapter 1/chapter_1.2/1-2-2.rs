// 1.2.2 

// Write an Interval1D client that takes an int value N as command-line argument,
// reads N intervals (each defined by a pair of double values) from standard input,
// and prints all pairs that intersect.

// public class Interval1D
// Interval1D(double lo, double hi) create an interval
// double length() length of the interval
// boolean contains(double x) does the interval contain x?
// boolean intersects(Interval1D that) does the interval intersect that?
// void draw() draw the interval on StdDraw

// https://docs.rs/intervals-general/latest/intervals_general/interval/enum.Interval.html

use rand::Rng;
use intervals_general::bound_pair::BoundPair;
use intervals_general::interval::Interval;

fn main() { 
  let mut rng = rand::thread_rng();
  // Create num_intervals of Intervals1D
  let mut intervals: Vec<Interval<f32>> = Vec::new();
  let num_intervals = 10;

  for i in 0..num_intervals {
    let x: f32 = rng.gen_range(-100.0..100.0);
    let y: f32 = rng.gen_range(-100.0..100.0);

    if x > y {
      let boundpair = BoundPair::new(y, x).unwrap();
      let closed_interval = Interval::Closed { bound_pair: boundpair };
      intervals.push(closed_interval);
    } else {
      let boundpair = BoundPair::new(x, y).unwrap();
      let closed_interval = Interval::Closed { bound_pair: boundpair };
      intervals.push(closed_interval);
    }
  }

  type IntervalPair = (Interval<f32>, Interval<f32>);
  let mut intersecting_intervals: Vec<IntervalPair> = Vec::new();

  for i in 0..num_intervals - 1 {
    for j in (i + 1)..num_intervals {
      let interval = intervals[i].intersect(&intervals[j]);
      if interval != Interval::Empty {
        intersecting_intervals.push( (intervals[i], intervals[j]) )
      }
    }
  }
  println!{"{:#?}", intersecting_intervals};
}