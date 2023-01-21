use std::cmp::min;

use ncurses::*;
const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

type Id = usize;

#[derive(Default)]
struct Ui {
    lst_curr: Option<Id>,
}

impl Ui {
    fn list_element(&mut self, input: &mut Vec<&str>, curr: usize) {
        for (row, todo) in input.iter().enumerate() {
            let pair = {
                if curr == row {
                    HIGHLIGHT_PAIR
                } else {
                    REGULAR_PAIR
                }
            };

            attron(COLOR_PAIR(pair));
            mv(row as i32, 0);
            addstr(*todo);
            attroff(COLOR_PAIR(pair));
        }
    }
    fn add_elements(&mut self, input: &mut Vec<&str>) {
        nocbreak();
        echo();

        let mut output: String = Default::default();

        let mut ch = getch();
        while ch as u8 as char != '\n' {
            output.push(ch as u8 as char);
            ch = getch();
        }

        noecho();
        cbreak();

    }
}

fn main() {
    initscr();
    noecho(); /* doesnt echo what you type */
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    let mut quit = false;
    let mut todos = vec![
        "Bey a bread",
        "Write a todo app",
        "Make a cup of tea"
    ];
    let mut dones = Vec::<&str>::new();
    let mut done_curr: usize = 0;
    let mut todo_curr: usize = 0;

    let mut ui = Ui::default();
    while !quit {

        ui.list_element(&mut todos, todo_curr);
        //ui.label("------------------------------");
        ui.list_element(&mut dones, done_curr);


        refresh();

        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            'w' | 'k' => if todo_curr > 0 { todo_curr -= 1; },
            's' | 'j' => todo_curr = min(todo_curr + 1, todos.len() - 1), /* ? */
            'a' => ui.add_elements(&mut todos),
            _ => {}
        }
    }

    endwin();
}

#[cfg(test)]
mod test;

