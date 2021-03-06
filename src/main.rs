mod ch1 {
    pub mod ex1;
    pub mod read_lines;
}

use ch1::ex1::filter_duplicates;
use ch1::ex1::filter_duplicates_sort_by_length;
use ch1::ex1::only_duplicates;
use ch1::ex1::queue_n_till_blank_line;
use ch1::ex1::reverse_lines;
use ch1::ex1::reverse_n_lines;

fn main() {
    // reverse_lines("./poem.txt");

    // reverse_n_lines("./poem.txt", 3);

    // queue_n_till_blank_line("./poem.txt", 3);

    // filter_duplicates("./poem.txt");

    // only_duplicates("./poem.txt");

    filter_duplicates_sort_by_length("./poem.txt");
}
