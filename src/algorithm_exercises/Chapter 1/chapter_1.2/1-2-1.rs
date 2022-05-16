// 1.2.2 

// Write an Interval1D client that takes an int value N as command-line argument,
// reads N intervals (each defined by a pair of double values) from standard input,
// and prints all pairs that intersect.

use euclid::Point2D;
use rand::Rng;

fn main() { 
  // Generate points
  let mut rng = rand::thread_rng();
  let mut points: Vec<Point2D<f32, f32>> = Vec::new();
  let num_points = 100;
  for x in 0..num_points {
    points.push( Point2D::new(rng.gen::<f32>(), rng.gen::<f32>()) )
  }

  // Compute minimum distance
  let mut min_dist: f32 = 1.50; // Choose initial value bigger than maximum possible length between two points in the unit square

  for x in 0..num_points - 1 {
    for y in (x + 1)..num_points {
      let calc_distance = distance(points[x], points[y]);
      if calc_distance < min_dist {min_dist = calc_distance;}
    }
  }

  println!{"Minimum distance is {:?}", min_dist};
}

// Function to calculate distance between two points
fn distance(point_1: Point2D<f32, f32>, point_2: Point2D<f32, f32>) -> f32 {
  let vector: Point2D<f32, f32> = Point2D::new(point_1.x - point_2.x, point_1.y - point_2.y);

  let x_squared = vector.x * vector.x;
  let y_squared = vector.y * vector.y;
  let distance = (x_squared + y_squared).sqrt();
  // println!{"{:?} {:?} {}", point_1, point_2, distance};
  distance
}