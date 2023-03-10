use ncurses::*;
use std::fs::File;
use std::io::{self, Write, BufRead};
use std::env;
use std::process;

const REGULAR_PAIR: i16 = 1;
const HIGHLIGHT_PAIR: i16 = 2;


/* ui {{{ */
/* TODO: add todos list and the dones list to the Ui struct */
#[derive(Default)]
struct Ui {
    list_curr: usize,
    row: usize,
    col: usize,
    layer: i32,
}

impl Ui {
    fn begin(&mut self, row:usize, col: usize) {
        self.row = row;
        self.col = col;
    }
    fn begin_list(&mut self, id: usize) {
        //assert!(self.list_curr.is_none(), "Nested lists are not allowed");
        self.list_curr = id;
    }
    fn list_element(&mut self, label: &str, id: usize, definition: &mut Vec<String>) -> bool {
        let id_curr = self.list_curr;
            //.expect("Not allowed to create list elements outside of list");/*TODO*/

        /* Calling self label function */
        self.label({
            /* if contains dictionary remove till end */
            if label.contains("[^") {
                &label[..label.chars().position(|c| c == '^').unwrap()-1]
            } else { label }
        }, {
            /* if the id is current then highlight it */
            if id_curr == id {
                /* TODO: Remove redundancy */
                if label.contains("[^") {
                    mvprintw(self.row as i32, self.col as i32 + 32, &definition[0]);
                }
                HIGHLIGHT_PAIR
            } else {
                REGULAR_PAIR
            }
        });

        return false;
    }
    fn label(&mut self, text: &str, pair: i16) {
        /* Moves cursor to position */
        mv(self.row as i32, self.col as i32);

        attron(COLOR_PAIR(pair));
        addstr(text);
        attroff(COLOR_PAIR(pair));

        /* Moves it one row down */
        self.row += 1;
    }
    fn insert_element(&mut self, input: &mut Vec<String>) {
        nocbreak();
        echo();

        let mut output: String = Default::default();

        let mut ch = getch();
        mv(self.row as i32, self.col as i32);
        while ch as u8 as char != '\n' {
            output.push(ch as u8 as char);
            ch = getch();
        }

        input.push(output);

        noecho();
        cbreak();

    }
    fn end_list(&mut self) {
        self.list_curr = 0;
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
fn list_move(list: &Vec<Vec<String>>, currlay: &mut i32, 
             direction: i32, list_curr: &mut usize) {
    let length: i32 = list.len() as i32;

    //if *currlay + direction != 0 {
    //    *currlay += direction;
    //}
    if *currlay + direction != length {
        *currlay += direction;
    }

    if *currlay < 0 { *currlay = 0 }

    *list_curr = 0;
}
/* }}} */
/* load'n save {{{ */
/* TODO: Rework load */
/* load state from file */
fn load_state(todos: &mut Vec<Vec<String>>, dones: &mut Vec<String>
              ,title: &mut Vec<String> ,file_path: &str) { 
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
    fn item(line: &str, num: usize) -> &str {
        return &line[num..];
    }

    let mut currlay: i32 = 0;

    /* TODO:
     * use std::fs::read_dir() to get the files in a direcotry.
     *
     * let paths = fn::read_dir("./").unwrap();
     * for paths in paths {} 
     */

    /* Future This Point to Directory */
    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        if line.as_ref().unwrap().starts_with("# ") {
            todos.push(vec![]);
            title.push(item(&line.unwrap(), 2).to_string());
            currlay += 1;
        } else if line.as_ref().unwrap().starts_with("- [ ] ") {
            todos[currlay as usize].push( item(&line.unwrap(), 6).to_string() );
        } else if line.as_ref().unwrap().starts_with("- [X] ") {
            dones.push( item(&line.unwrap(), 6).to_string() );
        }
    }
}

fn save_state(todos: &Vec<Vec<String>>, dones: &Vec<String>
              ,file_path: &str, title: &Vec<String> ){
    let mut file = File::create(file_path).unwrap();
    for x in 0..todos.len() {
        if title[x] != "" {
            writeln!(file, "# {}", title[x]).unwrap();
        }
        for todo in todos[x].iter() {
            writeln!(file, "- [ ] {}", todo).unwrap();
        }
    }
    writeln!(file, "").unwrap();
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
    let mut title: Vec<String> = vec!["".to_string()];
    let mut definition: Vec<String> = vec!["This is a temperary number1:todo!()".to_string()];
    //todos.push(vec!["TEST99".to_string()]);
    let mut dones: Vec<String> = Vec::<String>::new();
    let mut done_curr: usize = 0;

    initscr();
    noecho(); // doesnt echo what you type 
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE); // no display cursor

    use_default_colors(); /* Get default colors to not change the background colors with -1 */
    start_color(); /* Start Colors */
    init_pair(REGULAR_PAIR, COLOR_WHITE, -1); /* Regular color */
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);
    /* Highlighting color in highlighting elements */


    let mut tab = Tab::Todo;        /* Initilize Tab enum */
    let mut ui = Ui::default();     /* Initilize Ui enum */

    load_state(&mut todos, &mut dones, &mut title, &file_path); /* Loads elements from file */

    /* Loops into infinity untill break */
    'main: loop {
        erase();
        ui.begin(0, 0);
        {
            //ui.notification();
            /* TODO improve ui (overhal needed) */
            match tab {
                Tab::Todo => ui.label(&format!("[TODO]: {}", title[ui.layer as usize]).to_string(), REGULAR_PAIR),
                Tab::Done => ui.label(" TODO : ", REGULAR_PAIR),
            }

            ui.begin_list(todo_curr);
            for (row, todo) in todos[ui.layer as usize].iter().enumerate() {
                ui.list_element(&format!("\t[ ] {}", todo), row, &mut definition);
            }
            ui.end_list();

            match tab {
                Tab::Todo => ui.label(" DONE : ", REGULAR_PAIR),
                Tab::Done => ui.label("[DONE]: ", REGULAR_PAIR),
            }
            ui.begin_list(done_curr);
            for (row, done) in dones.iter().enumerate() {
                ui.list_element(&format!("\t[X] {}", done), row, &mut definition);
            }
            ui.end_list();
        }
        ui.end();


        refresh();

        let key = getch();
        match key as u8 as char { 
            /* Convert key to u8, to correctly.
             * Convert into a character. */

            'q' => break 'main,

            /* TODO: Find better method of matching. */
            'k' => match tab {
                Tab::Todo => list_up(&mut todo_curr),
                Tab::Done => list_up(&mut done_curr), 
            },
            'j' => match tab {
                Tab::Todo => list_down(&todos[ui.layer as usize], &mut todo_curr),
                Tab::Done => list_down(&dones, &mut done_curr), 
            },

            'l' => match tab {
                Tab::Todo => { 
                    list_move(&mut todos, &mut ui.layer, 1, &mut ui.list_curr);
                },
                Tab::Done => {}, 
            },
            'h' => match tab {
                Tab::Todo => {
                    list_move(&mut todos, &mut ui.layer, -1, &mut ui.list_curr);
                },
                Tab::Done => {}, 
            },

            'i' => match tab {
                Tab::Todo => ui.insert_element(&mut todos[ui.layer as usize]),
                Tab::Done => ui.insert_element(&mut dones),
            }

            'D' => match tab {
                Tab::Todo => list_delete(&mut todos[ui.layer as usize], &mut todo_curr),
                Tab::Done => list_delete(&mut dones, &mut done_curr),
            }

            '\n' => match tab {
                Tab::Todo => list_transfer(&mut dones, &mut todos[ui.layer as usize], &mut todo_curr),
                Tab::Done => list_transfer(&mut todos[ui.layer as usize], &mut dones, &mut done_curr),
            },
            '\t' => { tab = tab.toggle(); },

            _ => {}
        }
    }

    save_state(&todos, &dones, &file_path, &title);

    endwin(); // Restore terminal to normal behavior
}

#[cfg(test)]
mod test;
