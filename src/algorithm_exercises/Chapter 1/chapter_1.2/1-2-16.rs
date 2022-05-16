// 1.2.16
// Rational numbers. Implement an immutable data type Rational for rational numbers that supports addition, subtraction, multiplication, and division.

#[derive(Debug)]
struct Rational {
  numerator: i32,
  denominator: i32,
}

impl Rational {
  pub fn new(num: i32, denom: i32) -> Self {
        assert!(denom != 0, "denom cannot be 0");
        Rational { numerator: num, denominator: denom }
  }
  fn plus(&self, other: Rational) -> Rational {
    simplify_rational (
      Rational {
        numerator: self.numerator * other.denominator + self.denominator * other.numerator,
        denominator: self.denominator * other.denominator
      })
  }
  fn minus(&self, other: Rational) -> Rational {
    simplify_rational (
      Rational {
        numerator: self.numerator * other.denominator - self.denominator * other.numerator,
        denominator: self.denominator * other.denominator
      })
  }
  fn times(&self, other: Rational) -> Rational {
    simplify_rational (
      Rational {
        numerator: self.numerator * other.numerator,
        denominator: self.denominator * other.denominator
      })
  }
  fn divides(&self, other: Rational) -> Rational {
    simplify_rational (
      Rational {
        numerator: self.numerator * other.denominator,
        denominator: self.denominator * other.numerator
      })
  }
  fn equals(&self, other: Rational) -> bool {
    let copy_self = Rational {
      numerator: self.numerator,
      denominator: self.denominator
    };
    let simplified_self = simplify_rational(copy_self);
    let simplified_other = simplify_rational(other);
    
    simplified_self.numerator == simplified_other.numerator && simplified_self.denominator == simplified_other.denominator
  }
  fn toString(&self) -> String {
    let mut number = self.numerator.to_string();
    let comma = "/";
    let denom = self.denominator.to_string();
    number.push_str(comma);
    number.push_str(&denom);
    number
  }
}

fn main() {
  let num1 = Rational { numerator: 10, denominator: 7 };
  let num2 = Rational { numerator: 3, denominator: 4 };
  println!{"{:?}", num1.minus(num2)};
  let x = Rational::new(2, 3);
  println!{"{:?}", x};
}

fn simplify_rational (num: Rational) -> Rational {
  let gcd = gcd(num.numerator, num.denominator);
  Rational { 
    numerator: num.numerator / gcd, 
    denominator: num.denominator / gcd
  }
}

fn gcd (p: i32, q: i32) -> i32 {
  if q == 0 {return p}
  let r = p % q;
  gcd(q, r)
}