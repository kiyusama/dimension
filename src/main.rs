use tokio::time::{sleep, Duration};

const WIDTH: usize = 99;
const HEIGHT: usize = 31;

const Z_NEAR: f32 = -1.0;
const Z_FAR: f32 = -200.0;
const DISTANCE_CAM: f32 = 100.0;

const ROTATE_SPEED_X: f32 = 0.1;
const ROTATE_SPEED_Y: f32 = 0.15;
const ROTATE_SPEED_Z: f32 = 0.2;

#[tokio::main]
async fn main() {
    let mut x_rad = 0.0;
    let mut y_rad = 0.0;
    let mut z_rad = 0.0;

    // print!("\x1b[2J"); // 画面をクリア
    loop {
        let mut grid: [[char; WIDTH]; HEIGHT] = [[' '; WIDTH]; HEIGHT];
        let mut z_buffer: [[f32; WIDTH]; HEIGHT] = [[10.0; WIDTH]; HEIGHT];

        update_grid(&mut grid, &mut z_buffer, x_rad, y_rad, z_rad);

        // 文字列の領域をあらかじめ確保
        let mut frame_image = String::with_capacity(WIDTH * HEIGHT * 4);
        for height in 0..HEIGHT {
            for width in 0..WIDTH {
                frame_image.push_str(&format!("{}", grid[height][width]));
            }
            frame_image.push_str(&format!("\n"));
        }
        print!("\x1b[H{}", frame_image); // 左上端に

        x_rad += ROTATE_SPEED_X;
        y_rad += ROTATE_SPEED_Y;
        z_rad += ROTATE_SPEED_Z;

        sleep(Duration::from_secs_f32(0.05)).await;
    }
}

fn update_grid(
    grid: &mut [[char; WIDTH]; HEIGHT],
    z_buffer: &mut [[f32; WIDTH]; HEIGHT],
    x_rad: f32,
    y_rad: f32,
    z_rad: f32,
) {
    let mut i = -99.5;
    while i <= 99.5 {
        let x = i;
        let y = 0.0;
        let z = 0.0;

        let x_rotated = rotate_x(x, y, z, x_rad, y_rad, z_rad);
        let y_rotated = rotate_y(x, y, z, x_rad, y_rad, z_rad);
        let z_rotated = rotate_z(x, y, z, x_rad, y_rad) - DISTANCE_CAM;

        let x_screen = to_x_screen(x_rotated, z_rotated) as usize;
        let y_screen = to_y_screen(y_rotated, z_rotated) as usize;
        let depth = to_z_buffer(z_rotated);

        if depth < z_buffer[y_screen][x_screen] {
            z_buffer[y_screen][x_screen] = depth;
            grid[y_screen][x_screen] = '@';
        }

        i += 0.001;
    }
}

// 回転行列で算出
fn rotate_x(x: f32, y: f32, z: f32, x_rad: f32, y_rad: f32, z_rad: f32) -> f32 {
    return x * y_rad.cos() * z_rad.cos()
        + y * x_rad.sin() * y_rad.sin() * z_rad.cos()
        + z * x_rad.cos() * y_rad.sin() * z_rad.cos()
        - y * x_rad.cos() * z_rad.sin()
        + z * x_rad.sin() * z_rad.sin();
}

fn rotate_y(x: f32, y: f32, z: f32, x_rad: f32, y_rad: f32, z_rad: f32) -> f32 {
    return x * y_rad.cos() * z_rad.sin()
        + y * x_rad.sin() * y_rad.sin() * z_rad.sin()
        + z * x_rad.cos() * y_rad.sin() * z_rad.sin()
        + y * x_rad.cos() * z_rad.cos()
        - z * x_rad.sin() * z_rad.cos();
}

fn rotate_z(x: f32, y: f32, z: f32, x_rad: f32, y_rad: f32) -> f32 {
    return -x * y_rad.sin() + y * x_rad.sin() * y_rad.cos() + z * x_rad.cos() * y_rad.cos();
}

fn to_x_screen(x: f32, z: f32) -> f32 {
    //透視投影変換とNDCへの変換、ViewPort座標への変換を行う
    return (WIDTH as f32 - (2.0 * Z_NEAR / z) * x) / 2.0;
}

fn to_y_screen(y: f32, z: f32) -> f32 {
    return (HEIGHT as f32 + (2.0 * Z_NEAR / z) * y) / 2.0;
}

// 深度バッファを計算
fn to_z_buffer(z: f32) -> f32 {
    return (Z_FAR * z - Z_FAR * Z_NEAR) / ((Z_FAR - Z_NEAR) * z);
}
