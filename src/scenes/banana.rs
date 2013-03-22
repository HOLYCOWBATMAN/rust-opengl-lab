// use loader::Face;
// use loader::FaceTriplet;
// use loader::Obj;

// pub fn render_obj(obj: &Obj)
// {
//     // glClear( GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT );

//     // glNewList(list, GL_COMPILE);

//     for obj.faces.each |&face| {
//         println(fmt!("face"));
//         for face.triplets.each |&triplet| {
//             println(fmt!("\t%?", triplet));
//         }
//     }

//     // glEndList();
// }

// fn draw_face(obj: &Obj, face: &Face)
// {
//     let tl = len(face.triplets);

//     if (tl == 3)
//     { // triangle
//         draw_tri(obj, face);
//     }
//     else if (tl == 4)
//     { // quad
//         draw_quad(obj, face);
//     }
//     else
//     {
//         fail!(fmt!());
//     }
// }

// fn draw_tri(obj: &Obj, face: &Face)
// {
//     let has_normals = len(face.normal) == 3;

//     if (has_normals)
//     { // with normals
//         glBegin(GL_TRIANGLES);
//         glNormal3f(normals[face.normal[0]].v[0], normals[face.normal[0]].v[1], normals[face.normal[0]].v[2]);
//         glVertex3f(vertices[face.vertex[0]].v[0], vertices[face.vertex[0]].v[1], vertices[face.vertex[0]].v[2]);
//         glNormal3f(normals[face.normal[1]].v[0], normals[face.normal[1]].v[1], normals[face.normal[1]].v[2]);
//         glVertex3f(vertices[face.vertex[1]].v[0], vertices[face.vertex[1]].v[1], vertices[face.vertex[1]].v[2]);
//         glNormal3f(normals[face.normal[2]].v[0], normals[face.normal[2]].v[1], normals[face.normal[2]].v[2]);
//         glVertex3f(vertices[face.vertex[2]].v[0], vertices[face.vertex[2]].v[1], vertices[face.vertex[2]].v[2]);
//         glEnd();
//     }
//     else
//     { // without normals -- evaluate normal on triangle
//         vertex v = (vertices[face.vertex[1]] - vertices[face.vertex[0]]).cross(vertices[face.vertex[2]] - vertices[face.vertex[0]]);
//         v.normalize();
//         glBegin(GL_TRIANGLES);
//         glNormal3f(v.v[0], v.v[1], v.v[2]);
//         glVertex3f(vertices[face.vertex[0]].v[0], vertices[face.vertex[0]].v[1], vertices[face.vertex[0]].v[2]);
//         glVertex3f(vertices[face.vertex[1]].v[0], vertices[face.vertex[1]].v[1], vertices[face.vertex[1]].v[2]);
//         glVertex3f(vertices[face.vertex[2]].v[0], vertices[face.vertex[2]].v[1], vertices[face.vertex[2]].v[2]);
//         glEnd();
//     }
// }

// fn draw_quad(obj: &Obj, face: &Face)
// {
//     if (face.normal.size() == 4)
//     { // with normals
//         glBegin(GL_QUADS);
//         glNormal3f(normals[face.normal[0]].v[0], normals[face.normal[0]].v[1], normals[face.normal[0]].v[2]);
//         glVertex3f(vertices[face.vertex[0]].v[0], vertices[face.vertex[0]].v[1], vertices[face.vertex[0]].v[2]);
//         glNormal3f(normals[face.normal[1]].v[0], normals[face.normal[1]].v[1], normals[face.normal[1]].v[2]);
//         glVertex3f(vertices[face.vertex[1]].v[0], vertices[face.vertex[1]].v[1], vertices[face.vertex[1]].v[2]);
//         glNormal3f(normals[face.normal[2]].v[0], normals[face.normal[2]].v[1], normals[face.normal[2]].v[2]);
//         glVertex3f(vertices[face.vertex[2]].v[0], vertices[face.vertex[2]].v[1], vertices[face.vertex[2]].v[2]);
//         glNormal3f(normals[face.normal[3]].v[0], normals[face.normal[3]].v[1], normals[face.normal[3]].v[2]);
//         glVertex3f(vertices[face.vertex[3]].v[0], vertices[face.vertex[3]].v[1], vertices[face.vertex[3]].v[2]);
//         glEnd();
//     }
//     else
//     { // without normals -- evaluate normal on quad
//         vertex v = (vertices[face.vertex[1]] - vertices[face.vertex[0]]).cross(vertices[face.vertex[2]] - vertices[face.vertex[0]]);
//         v.normalize();
//         glBegin(GL_QUADS);
//         glNormal3f(v.v[0], v.v[1], v.v[2]);
//         glVertex3f(vertices[face.vertex[0]].v[0], vertices[face.vertex[0]].v[1], vertices[face.vertex[0]].v[2]);
//         glVertex3f(vertices[face.vertex[1]].v[0], vertices[face.vertex[1]].v[1], vertices[face.vertex[1]].v[2]);
//         glVertex3f(vertices[face.vertex[2]].v[0], vertices[face.vertex[2]].v[1], vertices[face.vertex[2]].v[2]);
//         glVertex3f(vertices[face.vertex[3]].v[0], vertices[face.vertex[3]].v[1], vertices[face.vertex[3]].v[2]);
//         glEnd();
//     }
// }