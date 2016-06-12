use std;

pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub special: f64
}

impl Color {
    pub fn new() -> Color {
        return Color { red: 0.0, blue: 0.0, green: 0.0, special: 0.0 };
    }

    pub fn add(&self, c2: &Color) -> Color {
        return Color {
            red: (self.red + c2.red),
            green: (self.green + c2.green),
            blue: (self.blue + c2.blue),
            special: (self.special + c2.special),
        };
    }
}

impl std::ops::Add for Color {
    type Output = Color;

    /// Adds two colors
    fn add(self, other: Color) -> Color {
        return Color {
            red: (self.red + other.red),
            green: (self.green + other.green),
            blue: (self.blue + other.blue),
            special: (self.special + other.special),
        };
    }
}
