use std;

pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub special: f64
}

const RGB_MAX: f64 = 255.0;

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

    pub fn multiply(&self, c2: &Color) -> Color {
        return Color {
            red: (self.red * c2.red),
            green: (self.green * c2.green),
            blue: (self.blue * c2.blue),
            special: (self.special * c2.special),
        };
    }

    pub fn average(c: &Color) -> Color {
        return Color {
            red: c.red / 2.0,
            green: c.green / 2.0,
            blue: c.blue / 2.0,
            special: c.special / 2.0,
        }
    }

    pub fn scalar(&self, scalar: f64) -> Color {
        return Color {
            red: self.red * scalar,
            green: self.green * scalar,
            blue: self.blue * scalar,
            special: self.special,
        }
    }

    pub fn clip(&mut self) {
        let sum_of_light = self.red + self.green + self.blue;
        let excess_light = sum_of_light - 3.0; // WTF?

        if excess_light > 0.0 {
            self.red = self.red + excess_light * (self.red / sum_of_light);
            self.green = self.green + excess_light * (self.green / sum_of_light);
            self.blue = self.blue + excess_light * (self.blue / sum_of_light);
        }

        self.red = self.red.min(1.0).max(0.0);
        self.green = self.green.min(1.0).max(0.0);
        self.blue = self.blue.min(1.0).max(0.0);
    }

    pub fn compute_pixel_color(c: &Color) -> Color {
        return Color {
            red: c.red * RGB_MAX,
            green: c.green * RGB_MAX,
            blue: c.blue * RGB_MAX,
            special: 0.0,
        }
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

impl std::ops::Mul<Color> for Color {
    type Output = Color;

    /// Multiplies two colors
    fn mul(self, other: Color) -> Color {
        return Color {
            red: (self.red * other.red),
            green: (self.green * other.green),
            blue: (self.blue * other.blue),
            special: (self.special * other.special),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn add_valid_input_should_return_correct_value() {
        let red_color = Color { red: 1.0, green: 0.0, blue: 0.0, special: 0.0 };
        let green_color = Color { red: 0.0, green: 1.0, blue: 0.0, special: 0.0 };

        let result = red_color + green_color;

        assert_eq!(1.0, result.red);
        assert_eq!(1.0, result.green);
        assert_eq!(0.0, result.blue);
        assert_eq!(0.0, result.special);
    }

    #[test]
    fn add_fn_valid_input_should_return_correct_value() {
        let red_color = Color { red: 1.0, green: 0.0, blue: 0.0, special: 0.0 };
        let green_color = Color { red: 0.0, green: 1.0, blue: 0.0, special: 0.0 };

        let result = red_color.add(&green_color);

        assert_eq!(1.0, result.red);
        assert_eq!(1.0, result.green);
        assert_eq!(0.0, result.blue);
        assert_eq!(0.0, result.special);
    }

    #[test]
    fn multiply_valid_input_should_return_correct_value() {
        let color_1 = Color { red: 0.5, green: 0.0, blue: 0.0, special: 0.0 };
        let color_2 = Color { red: 0.3, green: 0.0, blue: 0.0, special: 0.0 };

        let result = color_1 * color_2;

        assert_eq!(0.15, result.red);
        assert_eq!(0.0, result.green);
        assert_eq!(0.0, result.blue);
        assert_eq!(0.0, result.special);
    }

    #[test]
    fn multiply_fn_valid_input_should_return_correct_value() {
        let color_1 = Color { red: 0.5, green: 0.0, blue: 0.0, special: 0.0 };
        let color_2 = Color { red: 0.3, green: 0.0, blue: 0.0, special: 0.0 };

        let result = color_1.multiply(&color_2);

        assert_eq!(0.15, result.red);
        assert_eq!(0.0, result.green);
        assert_eq!(0.0, result.blue);
        assert_eq!(0.0, result.special);
    }

    #[test]
    fn average_valid_input_should_return_correct_value() {
        let color = Color { red: 0.5, green: 1.0, blue: 0.25, special: 0.75 };

        let result = Color::average(&color);

        assert_eq!(0.25, result.red);
        assert_eq!(0.5, result.green);
        assert_eq!(0.125, result.blue);
        assert_eq!(0.375, result.special);
    }

    #[test]
    fn scalar_valid_input_should_return_correct_value() {
        let color = Color { red: 0.5, green: 1.0, blue: 0.25, special: 0.75 };

        let result = color.scalar(3.0);

        assert_eq!(1.5, result.red);
        assert_eq!(3.0, result.green);
        assert_eq!(0.75, result.blue);
        assert_eq!(0.75, result.special);
    }

    #[test]
    fn compute_pixel_color_valid_input_should_return_correct_value() {
        let color = Color { red: 0.5, green: 1.0, blue: 0.25, special: 0.75 };

        let result = Color::compute_pixel_color(&color);

        assert_eq!(127.5, result.red);
        assert_eq!(255.0, result.green);
        assert_eq!(63.75, result.blue);
        assert_eq!(0.0, result.special);
    }

    #[test]
    fn clip_nonclippable_input_should_not_clip() {
        let mut color = Color { red: 0.5, green: 0.3, blue: 0.25, special: 0.75 };
        color.clip();

        assert_eq!(0.5, color.red);
        assert_eq!(0.3, color.green);
        assert_eq!(0.25, color.blue);
        assert_eq!(0.75, color.special);
    }

    #[test]
    fn clip_clippable_input_should_clip() {
        let mut color = Color { red: 1.5, green: 2.0, blue: 4.25, special: 0.75 };
        color.clip();

        assert_eq!(1.0, color.red);
        assert_eq!(1.0, color.green);
        assert_eq!(1.0, color.blue);
        assert_eq!(0.75, color.special);
    }
}
