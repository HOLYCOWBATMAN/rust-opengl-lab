// use core::io;
use core::path;
use core::str;
use core::str::raw;
use core::io::ReaderUtil;

pub struct Obj
{
    vertices: ~[float],
    normals: ~[float],
    texcoords: ~[float],
    faces: ~[float],
    faceValence: u16
}

const KEY_V: &static/str  = "v";
const KEY_VT: &static/str = "vt";
const KEY_VN: &static/str = "vn";
const KEY_F: &static/str  = "f";

impl Obj
{
    static fn parse(file_path: ~str) -> Obj
    {
        let pth = path::Path(file_path);
        let file_result = io::file_reader(&pth);

        let rdr = match file_result
        {
            Ok(rdr) => rdr,
            Err(err) => fail!(fmt!("failed to read file: %s", err))
        };

        let mut ln: ~str;

        let mut obj = Obj{
            vertices: ~[],
            normals: ~[],
            texcoords: ~[],
            faces: ~[],
            faceValence: 0
        };

        while !rdr.eof()
        {
            ln  = rdr.read_line();

            if !ln.is_empty()
            {
                parse_line(&mut obj, ln);
            }
        }

        return obj;
    }
}

fn parse_line(data: &mut Obj, line: &str)
{
    let (key, xs_line) = next_word(line);

    if key.len() == 0
    {
        return;
    }

    if(key == KEY_V.to_str())
    {
        next_flts(3, xs_line, &mut data.vertices);
    }
    else if(key == KEY_VT.to_str())
    {
        next_flts(2, xs_line, &mut data.texcoords);
    }

    // else if(key == key_vn)
    // {
    // //     // io::println("key_vn");
    // //     // line_to_vec_float(2, ln, &mut obj.normals);
    // }
    // else if(key == key_f)
    // {
    // //     // io::println("key_f");
    // }
}

fn next_word(line: &str) -> (~str, ~str)
{
    match next_token(line, |c| char::is_whitespace(c))
    {
        Some(ps) => ps,
        None => fail!(fmt!("Parse failed for line: %s", line))
    }
}

fn next_flt(line: &str) -> (float, ~str)
{
    match next_word(line)
    {
        (key, rest) =>
            match float::from_str(key)
            {
                Some(flt) => (flt, rest),
                None => fail!(fmt!("cannot convert string: \"%s\" to float", line))
            }
    }
}

fn next_flts(amount: uint, line: &str, store: & mut ~[float]) -> ~str
{
    let mut index: uint = 0;
    let mut xs_line: ~str = line.to_str();

    while index < amount
    {
        let (flt, nx_line) = next_flt(xs_line);
        store.push(flt);
        xs_line = nx_line;
        index += 1u;
    }

    return xs_line;
}

fn next_token(s: &str, sepfn: fn(cc: char) -> bool) -> Option<(~str, ~str)>
{
    let l       = str::len(s);
    let mut i = 0u, nx = 0u, stop = false;

    if (l == 0)
    {
        return None;
    }

    loop
    {
        if i < l
        {
            let str::CharRange {ch, next} = str::char_range_at(s, i);
            stop = sepfn(ch);
            nx = next;
        }
        else if i >= l
        {
            stop = true;
        }

        if stop
        {
            let xs_len = l-i;
            let tkn = unsafe { raw::slice_bytes(s, 0u, i) };

            let xs = match xs_len
            {
                0 => ~"",
                _ => unsafe { raw::slice_bytes(s, i+1u, l) }
            };

            return Some((tkn, xs));
        }

        i = nx;
    }
}


fn test_setup() -> ~Obj
{
    return ~Obj
    {
        vertices: ~[],
        normals: ~[],
        texcoords: ~[],
        faces: ~[],
        faceValence: 0
    };
}

#[test]
fn test_parse_v_line()
{
    let line = "v 63.035789 14.539266 -173.554443";
    let mut data = test_setup();
    parse_line(data, line);

    fail_unless!(vec::len(data.vertices) == 3);
    fail_unless!(eq(data.vertices[0], 63.035789f));
    fail_unless!(eq(data.vertices[1], 14.539266f));
    fail_unless!(eq(data.vertices[2], -173.554443f));
}

#[test]
fn test_parse_vt_line()
{
    let line = "vt 0.406606 0.637478 0";
    let mut data = test_setup();
    parse_line(data, line);

    fail_unless!(vec::len(data.texcoords) == 2);
    fail_unless!(eq(data.texcoords[0], 0.406606));
    fail_unless!(eq(data.texcoords[1], 0.637478));
}

fn eq(a: float, b: float) -> bool
{
    return float::abs(a - b) <= 1e-6;
}

#[test]
fn test_next_word()
{
    let words = ~["word_one", "word_two", "word_three"];
    let line = str::connect_slices(words, " ");

    let (word, xs_line) = next_word(line);
    fail_unless!(word == words[0].to_str());

    let (word, xs_line) = next_word(xs_line);
    fail_unless!(word == words[1].to_str());

    let (word, _) = next_word(xs_line);
    fail_unless!(word == words[2].to_str());
}