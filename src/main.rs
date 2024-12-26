use tokio::time::{sleep, Duration};

const WIDTH: usize = 99;
const HEIGHT: usize = 31;

#[tokio::main]
async fn main() {
    let mut grid: [i32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];

    print!("\x1b[2J"); // 画面をクリア
    loop {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                print_cell(grid[y * WIDTH + x]);
            }
            println!();
        }
        print!("\x1b[H"); // 左上端に
        update_grid(&mut grid);

        sleep(Duration::from_secs(5)).await;
    }
}

fn update_grid(grid: &mut [i32]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            grid[y * WIDTH + x] += 1;
        }
    }
}

fn print_cell(value: i32) {
    print!("\x1b[31m{}\x1b[0m", value % 10);
}
