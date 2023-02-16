use ncurses::*;
use std::fs::File;
use std::io::{self, Write, BufRead};
use std::env;
use std::process;

const REGULAR_PAIR: i16 = 1;
const HIGHLIGHT_PAIR: i16 = 2;

type Id = usize;

/* ui {{{ */

/* TODO: add todos list and the dones list to the Ui struct */
#[derive(Default)]
struct Ui {
    list_curr: Option<Id>,
    row: usize,
    col: usize,
    layer: usize,
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

        let mut output: String = text.to_string();
        if output.contains("[^") {
            loop {
                if output.contains("[^") { output.pop();
                } else { 
                    output.pop();
                    break;
                }
            }
        }
        addstr(&output);

        attroff(COLOR_PAIR(pair));
        self.row += 1;
    }
    fn insert_element(&mut self, input: &mut Vec<String>) {
        nocbreak();
        echo();

        let mut output: String = Default::default();

        let mut ch = getch();
        mv((self.row as i32), (self.col as i32));
        while ch as u8 as char != '\n' {
            output.push(ch as u8 as char);
            ch = getch();
        }

        input.push(output);

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
fn list_left(currlay: &mut usize) {
    if *currlay > 0 {
        *currlay -= 1;
    }
}
fn list_down(list: &Vec<String>, list_curr: &mut usize) {
    if *list_curr + 1 < list.len() {
        *list_curr += 1;
    }
}
fn list_right(list: &Vec<Vec<String>>, currlay: &mut usize) {
    if *currlay + 1 < list.len() {
        *currlay += 1;
    }
}
/* }}} */
/* load'n save {{{ */
/* TODO: Rework load */
/* parse input from file */
fn parse_item(line: &str) -> Option<(Tab, &str)> {
    let todo_prefix = "- [ ] ";
    let done_prefix = "- [X] ";
    let commit_prefix = "# ";
    
    if line.starts_with(commit_prefix) {
        return None;
    } else if line.starts_with(todo_prefix) {
        return Some((Tab::Todo, &line[todo_prefix.len()..]))
    } else if line.starts_with(done_prefix) {
        return Some((Tab::Done, &line[done_prefix.len()..]))
    }

    return None;
}
/* load state from file */
fn load_state(todos: &mut Vec<Vec<String>>, dones: &mut Vec<String>
              ,file_path: &str) { 

    let mut currlay: i32 = 0;

    /* TODO:
     * use std::fs::read_dir() to get the files in a direcotry.
     *
     * let paths = fn::read_dir("./").unwrap();
     * for paths in paths {} 
     */

    /* Future This Point to Directory */
    let file = File::open(file_path).unwrap();

    for line in io::BufReader::new(file).lines() {
        match parse_item(&line.unwrap()) {
            Some((Tab::Todo, title)) => todos[currlay as usize].push(title.to_string()),
            Some((Tab::Done, title)) => dones.push(title.to_string()),
            None => {
                currlay += 1;
                break;
            }
            _ => {},
        }
    }

}

fn save_state(todos: &Vec<Vec<String>>, dones: &Vec<String>
              ,file_path: &str){
    let mut file = File::create(file_path).unwrap();
    for x in 0..todos.len() {
        for todo in todos[x].iter() {
            writeln!(file, "- [ ] {}", todo).unwrap();
        }
    }
    for done in dones.iter() {
        writeln!(file, "- [X] {}", done).unwrap();
    }
}
/* }}} */
/* list edit {{{ */
fn list_delete(list: &mut Vec<String>, curr: &mut usize) {
    if *curr < list.len() {
        list.remove(*curr);
        if *curr >= list.len() && !list.is_empty() {
            *curr = list.len() - 1;
        }
    }
}
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
/* }}} */


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

    let mut todos: Vec<Vec<String>> = vec![vec![]];
    let mut todo_curr: usize = 0;
    let mut dones: Vec<String> = Vec::<String>::new();
    let mut done_curr: usize = 0;

    initscr();
    noecho(); // doesnt echo what you type 
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE); // no display cursor

    use_default_colors();
    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, -1);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);


    let mut tab = Tab::Todo;
    let mut quit = false;

    let mut ui = Ui::default();
    load_state(&mut todos, &mut dones, &file_path);
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
            for (row, todo) in todos[ui.layer].iter().enumerate() {
                ui.list_element(&format!("\t[ ] {}", todo), row);
            }
            ui.end_list();

            match tab {
                Tab::Todo => ui.label(" DONE : ", REGULAR_PAIR),
                Tab::Done => ui.label("[DONE]: ", REGULAR_PAIR),
            }
            ui.begin_list(done_curr);
            for (row, done) in dones.iter().enumerate() {
                ui.list_element(&format!("\t[X] {}", done), row);
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
                Tab::Todo => list_down(&todos[ui.layer], &mut todo_curr),
                Tab::Done => list_down(&dones, &mut done_curr), 
            },

            'l' => match tab {
                Tab::Todo => list_right(&mut todos, &mut ui.layer),
                Tab::Done => {}, 
            },
            'h' => match tab {
                Tab::Todo => list_left(&mut ui.layer),
                Tab::Done => {}, 
            },

            //'a' => vw_printw(initscr(), "da{}", "test"),
            '\n' => match tab {
                Tab::Todo => list_transfer(&mut dones, &mut todos[ui.layer], &mut todo_curr),
                Tab::Done => list_transfer(&mut todos[ui.layer], &mut dones, &mut done_curr),
            },
            'i' => match tab {
                Tab::Todo => ui.insert_element(&mut todos[ui.layer]),
                Tab::Done => ui.insert_element(&mut dones),
            }
            'D' => match tab {
                Tab::Todo => list_delete(&mut todos[ui.layer], &mut todo_curr),
                Tab::Done => list_delete(&mut dones, &mut done_curr),
            }
            '\t' => { tab = tab.toggle(); },
            _ => {}
        }
    }

    save_state(&todos, &dones, &file_path);

    endwin(); // Restore terminal to normal behavior
}

#[cfg(test)]
mod test;
