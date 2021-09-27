macro_rules! err {
    ($($tt: tt)*) => {
        eprintln!($($tt)*);
        std::process::exit(1);
    };
}

pub(crate) fn parse_header_line(line: &str) -> (i32, i32, u32, u32, u32) {
    fn parse_header_line(line: &str) -> Option<(i32, i32, u32, u32, u32)> {
        if let &[origin_x, origin_y, width, height, scale] =
            line.split_ascii_whitespace().collect::<Vec<_>>().as_slice()
        {
            Some((
                origin_x.parse().ok()?,
                origin_y.parse().ok()?,
                width.parse().ok()?,
                height.parse().ok()?,
                scale.parse().ok()?,
            ))
        } else {
            None
        }
    }

    if let Some(v) = parse_header_line(line) {
        v
    } else {
        err!("expected header: <origin_x> <origin_y> <width> <height> <scale>");
    }
}

pub(crate) enum IdOrNum<'a> {
    Id(&'a str),
    Num(i32),
}

macro_rules! err_if {
    ($cond: expr, $($tt: tt)*) => {
        if $cond {
            err!($($tt)*);
        }
    };
}

pub(crate) fn parse_line<'a>(line: &'a str, num: usize) -> Line<'a> {
    let line = line.trim_start();
    let (line, id) = identifier(line, num);
    let (line, assign_to, id) = if let Some(line) = line.strip_prefix("=") {
        let (line, id2) = identifier(line.trim_start(), num);
        (line, Some(id), id2)
    } else {
        (line, None, id)
    };

    let mut line = match_char(line, '(', num);
    let mut args = vec![];

    loop {
        let (line1, id_or_num) = identifier_or_number(line, num);
        args.push(id_or_num);
        if test_first(line1, |x| x == ',') {
            line = match_char(line1, ',', num);
            continue;
        } else {
            line = match_char(line1, ')', num);
            break;
        }
    }

    let _ = line;

    Line {
        assign_to,
        func: id,
        args,
    }
}

pub(crate) struct Line<'a> {
    pub(crate) assign_to: Option<&'a str>,
    pub(crate) func: &'a str,
    pub(crate) args: Vec<IdOrNum<'a>>,
}

fn match_char(str: &str, expect: char, num: usize) -> &str {
    let mut iter = str.char_indices();
    if let Some((_, c)) = iter.next() {
        let i = iter.next().map(|x| x.0).unwrap_or(str.len());
        err_if!(
            c != expect,
            "unexpected char/EOL at line {}, found {}, expecting {}",
            num,
            c,
            expect,
        );
        str.split_at(i).1.trim_start()
    } else {
        err!("unexpected char/EOL at line {}, expecting {}", num, expect);
    }
}

fn test_first<F: FnOnce(char) -> bool>(str: &str, test: F) -> bool {
    str.chars().nth(0).map(test).unwrap_or(false)
}

fn identifier_or_number(input: &str, num: usize) -> (&str, IdOrNum<'_>) {
    err_if!(
        !test_first(input, |x| x.is_alphanumeric() || x == '-'),
        "unexpected char/EOL at line {}, expecting identifier or number",
        num,
    );
    if test_first(input, |x| x.is_ascii_alphabetic()) {
        let (rest, ident) = identifier(input, num);
        (rest, IdOrNum::Id(ident))
    } else {
        let (rest, num) = number(input, num);
        (rest, IdOrNum::Num(num))
    }
}

fn identifier(input: &str, num: usize) -> (&str, &str) {
    err_if!(
        !test_first(input, char::is_alphabetic),
        "unexpected char/EOL at line {}, expecting identifier",
        num,
    );
    let i = input
        .find(|c: char| !c.is_ascii_alphanumeric())
        .unwrap_or(input.len());
    let (ident, rest) = input.split_at(i);
    (rest.trim_start(), ident)
}

fn number(input: &str, num: usize) -> (&str, i32) {
    err_if!(
        !test_first(input, |x| x.is_numeric() || x == '-'),
        "unexpected char/EOL at line {}, expecting number",
        num
    );
    let mut first = false;
    let i = input
        .find(|c: char| {
            let f = (first && c == '-') || !c.is_numeric();
            first = false;
            f
        })
        .unwrap_or(input.len());
    let (num, rest) = input.split_at(i);
    (rest.trim_start(), num.parse().expect("unexpect token"))
}
