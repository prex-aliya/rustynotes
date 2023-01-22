use ncurses::*;
use std::fs::File;
use std::io::{self, Write, BufRead};
use std::env;
use std::process;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

type Id = usize;

/* ui {{{ */
#[derive(Default)]
struct Ui {
    list_curr: Option<Id>,
    row: usize,
    col: usize,
}

impl Ui {
    fn begin(&mut self, row:usize, col: usize) {
        self.row = row;
        self.col = col;
    }
    fn begin_list(&mut self, id: Id) {
        assert!(self.list_curr.is_none(), "Nested lists are not allowed");
        self.list_curr = Some(id);
    }
    fn list_element(&mut self, label: &str, id: Id) -> bool {
        let id_curr = self.list_curr
            .expect("Not allowed to create list elements outside of list");

        self.label(label, {
            if id_curr == id {
                HIGHLIGHT_PAIR
            } else {
                REGULAR_PAIR
            }
        });

        return false;
    }
    fn label(&mut self, text: &str, pair: i16) {
        mv(self.row as i32, self.col as i32);
        attron(COLOR_PAIR(pair));
        addstr(text);
        attroff(COLOR_PAIR(pair));
        self.row += 1;
    }
    fn insert_element(&mut self, input: &mut Vec<String>) {
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
    fn end_list(&mut self) {
        self.list_curr = None;
    }
    fn end(&mut self) {
    }
}
/* }}} */
/* tab {{{ */
#[derive(Debug)]
enum Tab {
    Todo,
    Done
}

impl Tab {
    fn toggle(&self) -> Self {
        match self {
            Tab::Todo => Tab::Done,
            Tab::Done => Tab::Todo,
        }
    }
}
/* }}} */
/* move {{{ */
fn list_up(list_curr: &mut usize) {
    if *list_curr > 0 {
        *list_curr -= 1;
    }
}
fn list_down(list: &Vec<String>, list_curr: &mut usize) {
    if *list_curr + 1 < list.len() {
        *list_curr += 1;
    }
}
/* }}} */
/* {{{ */
fn save_state(todos: &Vec<String>, dones: &Vec<String>
              ,file_path: &str){
    let mut file = File::create(file_path).unwrap();
    for todo in todos.iter() {
        writeln!(file, "TODO:{}", todo).unwrap();
    }
    for done in dones.iter() {
        writeln!(file, "DONE:{}", done).unwrap();
    }
}
/* }}} */

fn list_transfer( dst: &mut Vec<String>, src: &mut Vec<String>,
                  src_curr: &mut usize
    ){
    if *src_curr < src.len() {
        dst.push(src.remove(*src_curr));
        if *src_curr >= src.len() && src.len() > 0 {
            *src_curr = src.len() -1;
        }
    }
}


fn main() {
    let mut args = env::args();
    args.next().unwrap();

    let file_path = match args.next() {
        Some(file_path) => file_path,
        None => {
            eprintln!("Usage: rustynotes [FILE] .. ");
            eprintln!("ERROR: the file path is not provided");
            process::exit(1)
        }
    };

    let mut todos: Vec<String> = vec![
        "Bey a bread".to_string(),
        "Write a todo app".to_string(),
        "Make a cup of tea".to_string(),
    ];
    let mut todo_curr: usize = 0;
    let mut dones: Vec<String> = vec![
        "Start the thing".to_string(),
        "the thing".to_string(),
    ];
    let mut done_curr: usize = 0;

    initscr();
    noecho(); /* doesnt echo what you type */
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);


    let mut tab = Tab::Todo;
    let mut quit = false;

    let mut ui = Ui::default();
    while !quit {
        erase();
        //ui.notification();
        ui.begin(0, 0);
        {
            match tab {
                Tab::Todo => ui.label("[TODO]: ", REGULAR_PAIR),
                Tab::Done => ui.label(" TODO : ", REGULAR_PAIR),
            }
            ui.begin_list(todo_curr);
            for (row, todo) in todos.iter().enumerate() {
                ui.list_element(&format!("- [ ] {}", todo), row);
            }
            ui.end_list();

            //ui.label("------------------------", REGULAR_PAIR);

            match tab {
                Tab::Todo => ui.label(" DONE : ", REGULAR_PAIR),
                Tab::Done => ui.label("[DONE]: ", REGULAR_PAIR),
            }
            ui.begin_list(done_curr);
            for (row, done) in dones.iter().enumerate() {
                ui.list_element(&format!("- [X] {}", done), row);
            }
            ui.end_list();
        }
        ui.end();


        refresh();

        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            'k' => match tab {
                Tab::Todo => list_up(&mut todo_curr),
                Tab::Done => list_up(&mut done_curr), 
            },
            'j' => match tab {
                Tab::Todo => list_down(&todos, &mut todo_curr),
                Tab::Done => list_down(&dones, &mut done_curr), 
            },
            'a' => done_curr += 1,
            '\n' => match tab {
                Tab::Todo => list_transfer(&mut dones, &mut todos, &mut todo_curr),
                Tab::Done => list_transfer(&mut todos, &mut dones, &mut done_curr),
            },
            '\t' => { tab = tab.toggle(); },
            'i' => match tab {
                Tab::Todo => ui.insert_element(&mut todos),
                Tab::Done => ui.insert_element(&mut dones),
            }
            _ => {}
        }
    }

    save_state(&todos, &dones, &file_path);

    endwin();
}

#[cfg(test)]
mod test;
