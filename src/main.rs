use clap::{Args, Parser, Subcommand};
use std::f64::consts;

/// Simple program to calculate areas of different forms
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Calculate area of a triangle
    Triangle(Triangle),
    /// Calculate area of a rectangle
    Rectangle(Rectangle),
    /// Calculate area of a square
    Square(Square),
    /// Calculate area of a circle
    Circle(Circle),
}

#[derive(Args)]
struct Square {
    side: f64,
}

#[derive(Args)]
struct Circle {
    radius: f64,
}

#[derive(Args)]
struct Triangle {
    base: f64,
    height: f64,
}

#[derive(Args)]
struct Rectangle {
    base: f64,
    height: f64,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Triangle(trig) => println!( "The area of the triangle is {:.2}", (trig.height * trig.base) / 2.),
        Commands::Rectangle(rect) => println!("The area of the rectangle is {:.2}", rect.height * rect.base),
        Commands::Square(sqr) => println!("The area of the square is {:.2}", sqr.side * sqr.side),
        Commands::Circle(circ) => println!("The area of the circle is {:.2}", consts::PI * (circ.radius * circ.radius)),
    }
}
