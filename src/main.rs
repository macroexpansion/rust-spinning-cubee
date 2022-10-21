use std::io::{self, Write};

#[derive(Default, Debug)]
struct Translation {
    A: f64,
    B: f64,
    C: f64,

    x: f64,
    y: f64,
    z: f64,

    ooz: f64,
    xp: isize,
    yp: isize,
}

impl Translation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn calculate_X(&self, i: f64, j: f64, k: f64) -> f64 {
        j * self.A.sin() * self.B.sin() * self.C.cos()
            - k * self.A.cos() * self.B.sin() * self.C.cos()
            + j * self.A.cos() * self.C.sin()
            + k * self.A.sin() * self.C.sin()
            + i * self.B.cos() * self.C.cos()
    }

    pub fn calculate_Y(&self, i: f64, j: f64, k: f64) -> f64 {
        j * self.A.cos() * self.C.cos() + k * self.A.sin() * self.C.cos()
            - j * self.A.sin() * self.B.sin() * self.C.sin()
            + k * self.A.cos() * self.B.sin() * self.C.sin()
            - i * self.B.cos() * self.C.sin()
    }

    pub fn calculate_Z(&self, i: f64, j: f64, k: f64) -> f64 {
        k * self.A.cos() * self.B.cos() - j * self.A.sin() * self.B.cos() + i * self.B.sin()
    }

    pub fn calculate_for_surface(
        &mut self,
        x: f64,
        y: f64,
        z: f64,
        character: char,
        distance_from_cam: f64,
        width: usize,
        height: usize,
        K1: f64,
        z_buffer: &mut [f64],
        buffer: &mut [char],
    ) {
        self.x = self.calculate_X(x, y, z);
        self.y = self.calculate_Y(x, y, z);
        self.z = self.calculate_Z(x, y, z) + distance_from_cam;
        self.ooz = 1.0 / self.z;
        self.xp = (width as f64 / 2.0 + K1 * self.ooz * self.x * 2.0) as isize;
        self.yp = (height as f64 / 2.0 + K1 * self.ooz * self.y) as isize;

        let idx = self.xp as usize + self.yp as usize * width;
        if idx >= 0 && idx < width * height {
            if self.ooz > z_buffer[idx] {
                z_buffer[idx] = self.ooz;
                buffer[idx] = character;
            }
        }
    }
}

#[derive(Debug)]
struct Config {
    cube_width: f64,
    width: usize,
    height: usize,
    background_ASCII_code: char,
    increment_speed: f64,
    distance_from_cam: f64,
    K1: f64,
}

fn main() {
    let mut translation = Translation::new();

    const config: Config = Config {
        cube_width: 10f64,
        width: 160,
        height: 44,
        background_ASCII_code: ' ',
        increment_speed: 0.6,
        distance_from_cam: 60f64,
        K1: 40f64,
    };

    io::stdout().write_all(b"\x1b[2J");
    loop {
        let mut z_buffer = [0f64; config.width * config.height];
        let mut buffer = [config.background_ASCII_code; config.width * config.height];

        let mut cube_X = -config.cube_width;
        while cube_X < config.cube_width {
            let mut cube_Y = -config.cube_width;
            while cube_Y < config.cube_width {
                translation.calculate_for_surface(
                    cube_X,
                    cube_Y,
                    -config.cube_width,
                    '#',
                    config.distance_from_cam,
                    config.width,
                    config.height,
                    config.K1,
                    &mut z_buffer,
                    &mut buffer,
                );
                cube_Y += config.increment_speed;
            }
            cube_X += config.increment_speed;
        }
        io::stdout().write_all(b"\x1b[H");
        let mut k = 0;
        while k < config.width * config.height {
            let ch = if k % config.width != 0 {
                buffer[k]
            } else {
                ' '
            };
            print!("{ch}");
            k += 1;
        }

        translation.A += 0.005;
        translation.B += 0.005;
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
