mod ch1 {
    pub mod ex1;
    pub mod read_lines;
}

use ch1::ex1::reverse_lines;

fn main() {
    reverse_lines("./poem.txt");
}
