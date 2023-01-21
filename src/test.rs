use super::*;


/*https://stackoverflow.com/questions/38995892/how-to-move-tests-into-a-separate-file-for-binaries-in-rusts-cargo*/

#[test]
fn test() {
    //open_file();
    testing();
}

use ncurses::*;
const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

fn testing() {
    initscr();

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    let mut quit = false;
    let mut todos = vec![
        "Bey a bread",
        "Write a todo app",
        "Make a cup of tea"
    ];
    let todo_curr: usize = 1;

    while !quit {
        for (row, todo) in todos.iter().enumerate() {
            let pair = {
                if todo_curr == row {
                    REGULAR_PAIR
                } else {
                    HIGHLIGHT_PAIR
                }
            };

            attron(COLOR_PAIR(pair));
            mv(row as i32, 0);
            attroff(COLOR_PAIR(pair));
            addstr(*todo);
        }

        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            _ => {}
        }
    }

    endwin();
}
