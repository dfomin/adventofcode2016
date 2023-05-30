use adventofcode2016::read_input;

struct Display {
    pixels: Vec<Vec<i32>>,
}

impl Display {
    fn new(width: usize, height: usize) -> Self {
        Display {
            pixels: vec![vec![0; width]; height],
        }
    }

    fn width(&self) -> usize {
        return self.pixels[0].len();
    }

    fn height(&self) -> usize {
        return self.pixels.len();
    }

    fn rect(&mut self, width: usize, height: usize) {
        for i in 0..height {
            for j in 0..width {
                self.pixels[i][j] = 1;
            }
        }
    }

    fn rotate_row(&mut self, row: usize, right: usize) {
        let right = right % self.width();

        let mut new_values = vec![0; self.width()];
        for i in 0..self.width() {
            new_values[i] = self.pixels[row][(i + self.width() - right) % self.width() as usize];
        }

        for i in 0..self.width() {
            self.pixels[row][i] = new_values[i];
        }
    }

    fn rotate_column(&mut self, column: usize, bottom: usize) {
        let bottom = bottom % self.height();

        let mut new_values = vec![0; self.height()];
        for i in 0..self.height() {
            new_values[i] =
                self.pixels[(i + self.height() - bottom) % self.height() as usize][column];
        }

        for i in 0..self.height() {
            self.pixels[i][column] = new_values[i];
        }
    }
}

fn apply_command(line: &str, display: &mut Display) {
    let parts = line.split(" ").collect::<Vec<_>>();
    match parts[0] {
        "rect" => {
            let params = parts[1]
                .split("x")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            display.rect(params[0], params[1]);
        }
        "rotate" => {
            if parts[1] == "row" {
                let row = parts[2].split("=").collect::<Vec<_>>()[1]
                    .parse::<usize>()
                    .unwrap();
                let right = parts[4].parse::<usize>().unwrap();
                display.rotate_row(row, right);
            } else {
                let column = parts[2].split("=").collect::<Vec<_>>()[1]
                    .parse::<usize>()
                    .unwrap();
                let bottom = parts[4].parse::<usize>().unwrap();
                display.rotate_column(column, bottom);
            }
        }
        _ => panic!("Unexpected command"),
    }
}

fn part1(input: &String) -> i32 {
    let mut display = Display::new(50, 6);
    for line in input.lines() {
        apply_command(line, &mut display);
    }

    return display
        .pixels
        .iter()
        .map(|x| -> i32 { x.iter().sum() })
        .sum();
}

fn part2(input: &String) -> String {
    let mut display = Display::new(50, 6);
    for line in input.lines() {
        apply_command(line, &mut display);
    }

    let mut result = String::new();
    for row in display.pixels {
        result.push_str(
            &row.iter()
                .map(|x| if *x > 0 { '#' } else { ' ' })
                .collect::<String>(),
        );
        result.push('\n');
    }

    return result;
}

fn main() {
    let input = read_input(8);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day8_part1() {
        let mut display = Display::new(7, 3);
        display.rect(3, 2);
        display.rotate_column(1, 1);
        display.rotate_row(0, 4);
        display.rotate_column(1, 1);

        assert_eq!(
            display.pixels,
            vec![
                [0, 1, 0, 0, 1, 0, 1],
                [1, 0, 1, 0, 0, 0, 0],
                [0, 1, 0, 0, 0, 0, 0]
            ]
        );
    }
}
