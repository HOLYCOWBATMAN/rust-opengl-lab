use core::path;
use core::str;
use core::str::raw;
use core::io::ReaderUtil;
use core::io::Reader;
// use util::println;

// types of .obj file entries supported
static KEY_V: &'static str      = "v";
static KEY_VT: &'static str     = "vt";
static KEY_VN: &'static str     = "vn";
static KEY_F: &'static str      = "f";
static KEY_USEMTL: &'static str = "usemtl";

// expected number of numerical elements in a given entry
pub static V_ELEM_COUNT: int  = 3;
pub static VT_ELEM_COUNT: int = 2;
pub static VN_ELEM_COUNT: int = 3;

pub struct ObjModel
{
    v: ~VertexSet<f32>,
    vn: ~VertexSet<f32>,
    vt: ~VertexSet<f32>,
    f: ~VertexSet<Face>,
    mtl: ~str
}

pub struct VertexSet<T>
{
    priv verts: ~[T],
    priv stride: uint,
    priv length: uint
}

impl<T> Container for VertexSet<T>
{
    #[inline(always)]
    fn len(&const self) -> uint { self.length }

    #[inline(always)]
    fn is_empty(&const self) -> bool { self.length == 0 }
}

impl<T> VertexSet<T>
{
    fn new() -> ~VertexSet<T>
    {
        ~VertexSet
        {
            verts: ~[],
            length: 0,
            stride: 0
        }
    }
}

impl ObjModel
{
    fn new() -> ~ObjModel
    {
        ~ObjModel {
            v: VertexSet::new(),
            vn: VertexSet::new(),
            vt: VertexSet::new(),
            f: VertexSet::new(),
            mtl: ~""
        }
    }
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
    fn new(v: int, vt: int, vn: int) -> FaceTriplet
    {
        return FaceTriplet
        {
            v_idx:  if v < 0 { v } else { (v - 1) * V_ELEM_COUNT },
            vt_idx: if vt < 0 { vt } else { (vt - 1) * VT_ELEM_COUNT },
            vn_idx: if vn < 0 { vn } else { (vn - 1) * VN_ELEM_COUNT }
        };
    }
}

pub fn obj_model_from_file(file_path: ~str) -> Result<~ObjModel, ~str>
{
    let pth = path::Path(file_path);
    do io::file_reader(&pth).map |rdr| {
        parse_obj(rdr)
    }
}

fn parse_obj(rdr: &@Reader) -> ~ObjModel
{
    let mut ln: ~str;
    let mut model = ObjModel::new();

    while !rdr.eof()
    {
        ln  = rdr.read_line();

        if !ln.is_empty()
        {
            parse_line(model, ln);
        }
    }

    model
}

fn parse_vertex_line(line: ~str, vset: &mut VertexSet<f32>)
{
    if vset.length == 0
    {
        vset.stride = count_words(line);
    }

    next_flts(vset.stride, line, &mut vset.verts);
    vset.length += 1;
}

fn parse_face_line(line: ~str, vset: &mut VertexSet<Face>)
{
    if vset.stride == 0
    {
        vset.stride = count_words(line);
    }

    let mut faceTriplets: ~[FaceTriplet] = ~[];

    for str::each_word(line) |wrd: &str|
    {
        let indices = parse_f_token(wrd);
        let faceTrp = match indices
        {
            [] =>          fail!(fmt!("f (face) entry needs at least one element: %s", line)),
            [v] =>         FaceTriplet::new(v, -1, -1),
            [v, vt] =>     FaceTriplet::new(v, vt, -1),
            [v, vt, vn] => FaceTriplet::new(v, vt, vn),
            _ =>           fail!(fmt!("f (face) should have no more than three elements: %s", line)),
        };

        faceTriplets.push(faceTrp);
    }

    let face = Face { triplets: faceTriplets };
    vset.verts.push(face);
    vset.length += 1;
}

fn parse_line(model: &mut ObjModel, line: &str)
{
    let (key, xs_line) = next_word(line);

    if key.len() == 0
    {
        return;
    }

    if str::eq_slice(key, KEY_USEMTL)
    {
        model.mtl = xs_line.trim().to_owned();
    }
    else if str::eq_slice(key, KEY_V)
    {
        parse_vertex_line(xs_line, model.v);
    }
    else if str::eq_slice(key, KEY_VN)
    {
        parse_vertex_line(xs_line, model.vn);
    }
    else if str::eq_slice(key, KEY_VT)
    {
        parse_vertex_line(xs_line, model.vt);
    }
    else if str::eq_slice(key, KEY_F)
    {
        parse_face_line(xs_line, model.f);
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

fn count_words(line: &str) -> uint
{
    let mut count: uint   = 0;
    let mut xs_line: ~str = line.to_str();

    loop {
        match next_token(xs_line, |c| char::is_whitespace(c))
        {
            Some((_, nx_line)) => {
                count   += 1u;
                xs_line = nx_line;
            },
            None => break
        }
    }

    return count;
}

fn parse_f_token(line: &str) -> ~[int]
{
    let mut indices = ~[];

    for line.each_split_char('/') |tk|
    {
        match int::from_str(tk)
        {
            Some(index) => indices.push(index),
            None => fail!(fmt!("cannot convert string: \"%s\" from line: \"%s\" to u32", tk, line))
        }
    }

    indices
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

fn next_flts(amount: uint, line: &str, store: &mut ~[f32]) -> ~str
{
    let mut index: uint   = 0;
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

fn next_token(s: &str, sepfn: &fn(cc: char) -> bool) -> Option<(~str, ~str)>
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
                0 => "",
                _ => unsafe { raw::slice_bytes(s, i+1u, l) }
            };

            return Some((
                tkn.to_owned(),
                xs.trim_left().to_owned()
            ));
        }

        i = nx;
    }
}

#[test]
fn test_parse_usemtl_line()
{
    let line = "usemtl banana";
    let mut model =  ObjModel::new();
    parse_line(model, line);

    assert!(model.mtl == ~"banana");
}

#[test]
fn test_parse_v_line()
{
    use std::cmp::FuzzyEq;
    let line = "v 63.035789 14.539266 -173.554443";
    let mut model = ObjModel::new();
    parse_line(model, line);

    assert!(vec::len(model.v.len) == 3);
    assert!(model.v[0].fuzzy_eq(&63.035789f32));
    assert!(model.v[1].fuzzy_eq(&14.539266f32));
    assert!(model.v[2].fuzzy_eq(&-173.554443f32));
}

#[test]
fn test_parse_vt_line()
{
    use std::cmp::FuzzyEq;
    let line = "vt 0.406606 0.637478 0";
    let mut model = ObjModel::new();
    parse_line(model, line);

    assert!(vec::len(model.vt) == 2);
    assert!(model.vt[0].fuzzy_eq(&0.406606f32));
    assert!(model.vt[1].fuzzy_eq(&0.637478f32));
}

#[test]
fn test_parse_vn_line()
{
    use std::cmp::FuzzyEq;
    let line = "vn 63.035789 14.539266 -173.554443";
    let mut model = ObjModel::new();
    parse_line(model, line);

    assert!(vec::len(model.vn) == 3);
    assert!(model.vn[0].fuzzy_eq(&63.035789f32));
    assert!(model.vn[1].fuzzy_eq(&14.539266f32));
    assert!(model.vn[2].fuzzy_eq(&-173.554443f32));
}

#[test]
fn test_parse_f_v_line()
{
    let line = "f 1 22 333";
    let mut model = ObjModel::new();

    parse_line(model, line);

    {
        let f_lst = &model.f;
        let face = &f_lst[0];

        assert!(f_lst.len() == 1u);
        assert!(vec::len(face.triplets) == 3);
        assert_face_eq(&face.triplets[0], 0, -1, -1);
        assert_face_eq(&face.triplets[1], 21, -1, -1);
        assert_face_eq(&face.triplets[2], 332, -1, -1);
    }
}

// the indices used in the "f" entry of an .obj format are one based

#[test]
fn test_parse_f_vvt_line()
{
    let line = "f 1/2 22/33 333/444";
    let mut model = ObjModel::new();
    parse_line(model, line);

    {
        let face = &model.faces[0];

        assert!(vec::len(model.triplets) == 3);
        assert_face_eq(&model.triplets[0], 0, 1, -1);
        assert_face_eq(&model.triplets[1], 21, 32, -1);
        assert_face_eq(&model.triplets[2], 332, 443, -1);
    }
}

#[test]
fn test_parse_f_vvtvn_line()
{
    let line = "f 1/2/3 22/33/44 333/444/555";
    let mut model = ObjModel::new();
    parse_line(model, line);

    {
        let face = &model.faces[0];

        assert!(vec::len(face.triplets) == 3);
        assert_face_eq(&face.triplets[0], 0, 1, 2);
        assert_face_eq(&face.triplets[1], 21, 32, 43);
        assert_face_eq(&face.triplets[2], 332, 443, 554);
    }
}

fn assert_face_eq(triplet: &FaceTriplet, v: int, vt: int, vn: int)
{
    assert!(triplet.v_idx == if v < 0 { v } else { v * V_ELEM_COUNT });
    assert!(triplet.vt_idx == if vt < 0 { vt } else { vt * VT_ELEM_COUNT });
    assert!(triplet.vn_idx == if vn < 0 { vn } else { vn * VN_ELEM_COUNT });
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
        let model = parse_obj(rdr);

        assert!(model.v.len() == 12);
        assert!(model.vn.len() == 0);
        assert!(model.vt.len() == 8);
        assert!(model.f.len() == 2);

        let f_0 = &model.f[0];
        assert!(f_0.triplets[0].v_idx == 0 * V_ELEM_COUNT);
        assert!(f_0.triplets[1].v_idx == 1 * V_ELEM_COUNT);
        assert!(f_0.triplets[2].v_idx == 2 * V_ELEM_COUNT);

        let f_1 = &model.faces[1];
        assert!(f_1.triplets[0].v_idx == 2 * V_ELEM_COUNT);
        assert!(f_1.triplets[1].v_idx == 3 * V_ELEM_COUNT);
        assert!(f_1.triplets[2].v_idx == 0 * V_ELEM_COUNT);
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

        assert!(mdl.v.len == 24);
        assert!(mdl.vn.len == 18);
        assert!(mdl.vt.len == 8);
        assert!(mdl.f.len == 12);
    }
}

#[test]
fn test_next_word()
{
    let words = ~["word_one", "word_two", "word_three"];
    let line = str::connect_slices(words, " ");

    let (word, xs_line) = next_word(line);
    assert!(word == words[0].to_str());

    let (word, xs_line) = next_word(xs_line);
    assert!(word == words[1].to_str());

    let (word, _) = next_word(xs_line);
    assert!(word == words[2].to_str());
}