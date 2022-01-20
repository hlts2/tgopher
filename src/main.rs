use std::io::{Stdout, Write};
use std::{thread, time};
use termion::{clear, cursor};

fn main() {
    // Ref: https://gist.github.com/belbomemo/b5e7dad10fa567a5fe8a
    let gopher_vec = vec![
        "         ,_---~~~~~----.          ",
        "  _,,_,*^____      _____``*g*\"*, ",
        r" / __/ /'     ^.  /      \ ^@q   f",
        "[  @f | @))    |  | @))   l  0 _/ ",
        r" \`/   \~____ / __ \_____/    \   ",
        "  |           _l__l_           I  ",
        "  }          [______]           I ",
        "  ]            | | |            | ",
        "  ]             ~ ~             | ",
        "  |                            |  ",
        "   |                           |  ",
    ];
    let max_len = gopher_vec[0].len() as u16;

    let mut out = std::io::stdout();
    clear_screen(&mut out);

    match termion::terminal_size() {
        Ok((x, _)) => {
            if x <= max_len {
                println!("can not output gopher....");
                return;
            }
            for i in (1..x - max_len).rev() {
                for (idx, e) in gopher_vec.iter().enumerate() {
                    write!(out, "{}", cursor::Goto(i, idx as u16)).unwrap();
                    write!(out, "{}", e).unwrap();
                    out.flush().unwrap();
                }
                thread::sleep(time::Duration::from_millis(20));
                clear_screen(&mut out);
            }
        }
        Err(_) => {
            return;
        }
    };
}

fn clear_screen(out: &mut Stdout) {
    write!(out, "{}", clear::All).unwrap();
    out.flush().unwrap();
}
