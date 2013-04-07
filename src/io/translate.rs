// use gl = opengles::gl3;
// use io::model;
// use io::model::ObjModel;

// pub fn model_to_flat_data(model: &ObjModel) -> (~[gl::GLfloat], ~[gl::GLuint])
// {
//     let v_lst            = &model.v;
//     let v_amt: uint      = model.v_len;
//     let v_stride: uint   = 3u;
//     let mut vi: uint     = 0u;

//     let vt_lst           = &model.vt;
//     let vt_amt: uint     = model.vt_len;
//     let vt_stride: uint  = 2u;
//     let mut vti: uint    = 0u;

//     let tgt_stride: uint = v_stride + vt_stride;
//     let mut tgti: uint   = 0;

//     let vtd_len           = v_amt + vt_amt;
//     let mut v_vt_data = vec::with_capacity(vtd_len);
//     unsafe { vec::raw::set_len(&mut v_vt_data, vtd_len); }

//     while(vi < v_amt)
//     {
//         v_vt_data[tgti]   = v_lst[vi];
//         v_vt_data[tgti+1] = v_lst[vi+1];
//         v_vt_data[tgti+2] = v_lst[vi+2];
//         v_vt_data[tgti+3] = vt_lst[vti];
//         v_vt_data[tgti+4] = vt_lst[vti+1];

//         vi   += v_stride;
//         vti  += vt_stride;
//         tgti += tgt_stride;
//     }

//     let f_lst          = &model.f;
//     let f_amt: uint    = f_lst.len();
//     let f_stride: uint = 3u;
//     let mut ei: uint   = 0u;
//     let mut fi: uint   = 0u;
//     let elem_len       = f_amt * f_stride;
//     let mut e_data     = vec::with_capacity(elem_len);
//     unsafe { vec::raw::set_len(&mut e_data, elem_len); }

//     while(fi < f_amt)
//     {
//         let f: &model::Face = f_lst[fi];

//         e_data[ei]   = f.triplets[0].v_idx / model::V_ELEM_COUNT as u32;
//         e_data[ei+1] = f.triplets[1].v_idx / model::V_ELEM_COUNT as u32;
//         e_data[ei+2] = f.triplets[2].v_idx / model::V_ELEM_COUNT as u32;

//         ei += f_stride;
//         fi += 1u;
//     }

//     (v_vt_data, e_data)
// }