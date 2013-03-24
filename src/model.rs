use core::path;
use core::str;
use core::str::raw;
use core::io::ReaderUtil;
use core::io::Reader;
// use util::println;

// types of .obj file entries supported
const KEY_V: &str      = "v";
const KEY_VT: &str     = "vt";
const KEY_VN: &str     = "vn";
const KEY_F: &str      = "f";
const KEY_USEMTL: &str = "usemtl";

// expected number of numerical elements in a given entry
pub const V_ELEM_COUNT: int  = 3;
pub const VT_ELEM_COUNT: int = 2;
pub const VN_ELEM_COUNT: int = 3;

pub struct ObjModel
{
    vertices: ~[f32],
    normals: ~[f32],
    texcoords: ~[f32],
    faces: ~[~Face],
    material: ~str
}

pub struct Face
{
    triplets: ~[FaceTriplet]
}

pub struct FaceTriplet
{
    v_idx: int,
    vt_idx: int,
    vn_idx: int
}

impl FaceTriplet
{
    static fn new(v: int, vt: int, vn: int) -> FaceTriplet
    {
        return FaceTriplet
        {
            v_idx:  if v < 0 { v } else { (v - 1) * V_ELEM_COUNT },
            vt_idx: if vt < 0 { vt } else { (vt - 1) * VT_ELEM_COUNT },
            vn_idx: if vn < 0 { vn } else { (vn - 1) * VN_ELEM_COUNT }
        };
    }
}

impl ObjModel
{

}

pub fn obj_model_from_file(file_path: ~str) -> ~ObjModel
{
    let pth = path::Path(file_path);
    let file_result = io::file_reader(&pth);

    let rdr = match file_result
    {
        Ok(rdr) => rdr,
        Err(err) => fail!(fmt!("failed to read file: %s", err))
    };

    parse_obj(rdr)
}

fn parse_obj(rdr: Reader) -> ~ObjModel
{
    let mut ln: ~str;

    let mut obj = ~ObjModel{
        vertices: ~[],
        normals: ~[],
        texcoords: ~[],
        faces: ~[],
        material: ~""
    };

    while !rdr.eof()
    {
        ln  = rdr.read_line();

        if !ln.is_empty()
        {
            parse_line(obj, ln);
        }
    }

    obj
}

fn parse_line(data: &mut ObjModel, line: &str)
{
    let (key, xs_line) = next_word(line);

    if key.len() == 0
    {
        return;
    }

    if str::eq_slice(key, KEY_USEMTL)
    {
        data.material = str::trim(xs_line)
    }
    else if str::eq_slice(key, KEY_V)
    {
        next_flts(3, xs_line, &mut data.vertices);
    }
    else if str::eq_slice(key, KEY_VN)
    {
        next_flts(3, xs_line, &mut data.normals);
    }
    else if str::eq_slice(key, KEY_VT)
    {
        next_flts(2, xs_line, &mut data.texcoords);
    }
    else if str::eq_slice(key, KEY_F)
    {
        let words = str::words(xs_line);
        let mut faceTriplets: ~[FaceTriplet] = ~[];

        for vec::each(words) |&wrd| {

            let indices = parse_f_token(wrd);
            let faceTrp = match indices
            {
                [] =>          fail!(fmt!("f (face) entry needs at least one element: %s", xs_line)),
                [v] =>         FaceTriplet::new(v, -1, -1),
                [v, vt] =>     FaceTriplet::new(v, vt, -1),
                [v, vt, vn] => FaceTriplet::new(v, vt, vn),
                _ =>           fail!(fmt!("f (face) should have no more than three elements: %s", xs_line)),
            };

            faceTriplets.push(faceTrp);
        }

        let face = ~Face { triplets: faceTriplets };

        data.faces.push(face);
    }
}

fn next_word(line: &str) -> (~str, ~str)
{
    match next_token(line, |c| char::is_whitespace(c))
    {
        Some(ps) => ps,
        None => fail!(fmt!("Parse failed for line: %s", line))
    }
}

fn parse_f_token(line: &str) -> ~[int]
{
    do vec::map(str::split(line, |c| c == '/')) |&tk|
    {
        match int::from_str(tk)
        {
            Some(index) => index,
            None => fail!(fmt!("cannot convert string: \"%s\" from line: \"%s\" to u32", tk, line))
        }
    }
}

fn next_flt(line: &str) -> (f32, ~str)
{
    match next_word(line)
    {
        (key, rest) =>
            match f32::from_str(key)
            {
                Some(flt) => (flt, rest),
                None => fail!(fmt!("cannot convert string: \"%s\" to f32", line))
            }
    }
}

fn next_flts(amount: uint, line: &str, store: & mut ~[f32]) -> ~str
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
    let l = str::len(s);
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

            return Some((tkn, str::trim_left(xs)));
        }

        i = nx;
    }
}

fn test_setup() -> ~ObjModel
{
    return ~ObjModel {
        vertices: ~[],
        normals: ~[],
        texcoords: ~[],
        faces: ~[],
        material:~ ""
    }
}

#[test]
fn test_parse_usemtl_line()
{
    let line = "usemtl banana";
    let mut data = test_setup();
    parse_line(data, line);

    fail_unless!(data.material == ~"banana");
}

#[test]
fn test_parse_v_line()
{
    let line = "v 63.035789 14.539266 -173.554443";
    let mut data = test_setup();
    parse_line(data, line);

    fail_unless!(vec::len(data.vertices) == 3);
    fail_unless!(eq(data.vertices[0], 63.035789f32));
    fail_unless!(eq(data.vertices[1], 14.539266f32));
    fail_unless!(eq(data.vertices[2], -173.554443f32));
}

#[test]
fn test_parse_vt_line()
{
    let line = "vt 0.406606 0.637478 0";
    let mut data = test_setup();
    parse_line(data, line);

    fail_unless!(vec::len(data.texcoords) == 2);
    fail_unless!(eq(data.texcoords[0], 0.406606f32));
    fail_unless!(eq(data.texcoords[1], 0.637478f32));
}

#[test]
fn test_parse_vn_line()
{
    let line = "vn 63.035789 14.539266 -173.554443";
    let mut data = test_setup();
    parse_line(data, line);

    fail_unless!(vec::len(data.normals) == 3);
    fail_unless!(eq(data.normals[0], 63.035789f32));
    fail_unless!(eq(data.normals[1], 14.539266f32));
    fail_unless!(eq(data.normals[2], -173.554443f32));
}

#[test]
fn test_parse_f_v_line()
{
    let line = "f 1 22 333";
    let mut data = test_setup();

    parse_line(data, line);

    {
        let f_lst = &data.faces;
        let face = &f_lst[0];

        fail_unless!(f_lst.len() == 1u);
        fail_unless!(vec::len(face.triplets) == 3);
        fail_unless_face_eq(&face.triplets[0], 0, -1, -1);
        fail_unless_face_eq(&face.triplets[1], 21, -1, -1);
        fail_unless_face_eq(&face.triplets[2], 332, -1, -1);
    }
}

// the indices used in the "f" entry of an .obj format are one based

#[test]
fn test_parse_f_vvt_line()
{
    let line = "f 1/2 22/33 333/444";
    let mut data = test_setup();
    parse_line(data, line);

    {
        let face = &data.faces[0];

        fail_unless!(vec::len(face.triplets) == 3);
        fail_unless_face_eq(&face.triplets[0], 0, 1, -1);
        fail_unless_face_eq(&face.triplets[1], 21, 32, -1);
        fail_unless_face_eq(&face.triplets[2], 332, 443, -1);
    }
}

#[test]
fn test_parse_f_vvtvn_line()
{
    let line = "f 1/2/3 22/33/44 333/444/555";
    let mut data = test_setup();
    parse_line(data, line);

    {
        let face = &data.faces[0];

        fail_unless!(vec::len(face.triplets) == 3);
        fail_unless_face_eq(&face.triplets[0], 0, 1, 2);
        fail_unless_face_eq(&face.triplets[1], 21, 32, 43);
        fail_unless_face_eq(&face.triplets[2], 332, 443, 554);
    }
}

fn fail_unless_face_eq(triplet: &FaceTriplet, v: int, vt: int, vn: int)
{
    fail_unless!(triplet.v_idx == if v < 0 { v } else { v * V_ELEM_COUNT });
    fail_unless!(triplet.vt_idx == if vt < 0 { vt } else { vt * VT_ELEM_COUNT });
    fail_unless!(triplet.vn_idx == if vn < 0 { vn } else { vn * VN_ELEM_COUNT });
}

fn eq(a: f32, b: f32) -> bool
{
    return f32::abs(a - b) <= 1e-6;
}

#[test]
fn test_file_parse()
{
    use core::io::with_str_reader;

    let file_txt = ~"\
        g quad\n\
        usemtl sample\n\
        v -0.5 0.5 0.0\n\
        v  0.5 0.5 0.0\n\
        v  0.5 -0.5 0.0\n\
        v -0.5 -0.5 0.0\n\
        vt 0.0 0.0\n\
        vt 1.0 0.0\n\
        vt 1.0 1.0\n\
        vt 0.0 1.0\n\
        f 1/1 2/2 3/3\n\
        f 3/3 4/4 1/1
    ";

    do with_str_reader(file_txt) |rdr| {
        let mdl = parse_obj(rdr);

        fail_unless!(mdl.vertices.len() == 12);
        fail_unless!(mdl.normals.len() == 0);
        fail_unless!(mdl.texcoords.len() == 8);
        fail_unless!(mdl.faces.len() == 2);

        let f_0 = &mdl.faces[0];
        fail_unless!(f_0.triplets[0].v_idx == 0 * V_ELEM_COUNT);
        fail_unless!(f_0.triplets[1].v_idx == 1 * V_ELEM_COUNT);
        fail_unless!(f_0.triplets[2].v_idx == 2 * V_ELEM_COUNT);

        let f_1 = &mdl.faces[1];
        fail_unless!(f_1.triplets[0].v_idx == 2 * V_ELEM_COUNT);
        fail_unless!(f_1.triplets[1].v_idx == 3 * V_ELEM_COUNT);
        fail_unless!(f_1.triplets[2].v_idx == 0 * V_ELEM_COUNT);
    }
}

#[test]
fn test_parse_file_unsupported_entries()
{
    use core::io::with_str_reader;

    let file_txt = ~"
        # cube.obj\n\
        #\n\

        o cube\n\
        mtllib cube.mtl\n\

        v -0.500000 -0.500000 0.500000\n\
        v 0.500000 -0.500000 0.500000\n\
        v -0.500000 0.500000 0.500000\n\
        v 0.500000 0.500000 0.500000\n\
        v -0.500000 0.500000 -0.500000\n\
        v 0.500000 0.500000 -0.500000\n\
        v -0.500000 -0.500000 -0.500000\n\
        v 0.500000 -0.500000 -0.500000\n\

        vt 0.000000 0.000000\n\
        vt 1.000000 0.000000\n\
        vt 0.000000 1.000000\n\
        vt 1.000000 1.000000\n\

        vn 0.000000 0.000000 1.000000\n\
        vn 0.000000 1.000000 0.000000\n\
        vn 0.000000 0.000000 -1.000000\n\
        vn 0.000000 -1.000000 0.000000\n\
        vn 1.000000 0.000000 0.000000\n\
        vn -1.000000 0.000000 0.000000\n\

        g cube\n\
        usemtl cube\n\
        s 1\n\
        f 1/1/1 2/2/1 3/3/1\n\
        f 3/3/1 2/2/1 4/4/1\n\
        s 2\n\
        f 3/1/2 4/2/2 5/3/2\n\
        f 5/3/2 4/2/2 6/4/2\n\
        s 3\n\
        f 5/4/3 6/3/3 7/2/3\n\
        f 7/2/3 6/3/3 8/1/3\n\
        s 4\n\
        f 7/1/4 8/2/4 1/3/4\n\
        f 1/3/4 8/2/4 2/4/4\n\
        s 5\n\
        f 2/1/5 8/2/5 4/3/5\n\
        f 4/3/5 8/2/5 6/4/5\n\
        s 6\n\
        f 7/1/6 1/2/6 5/3/6\n\
        f 5/3/6 1/2/6 3/4/6\n\
    ";

    do with_str_reader(file_txt) |rdr| {
        let mdl = parse_obj(rdr);

        fail_unless!(mdl.vertices.len() == 24);
        fail_unless!(mdl.normals.len() == 18);
        fail_unless!(mdl.texcoords.len() == 8);
        fail_unless!(mdl.faces.len() == 12);
    }
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