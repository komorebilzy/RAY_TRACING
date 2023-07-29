use crate::*;
use tobj::{load_obj, LoadOptions};
pub fn load<M: Material + Clone + 'static>(pathname: &str, mat: M, scale: f64) -> HitableList {
    let (models, _) = tobj::load_obj(
        pathname,
        &LoadOptions {
            single_index: false,
            triangulate: true,
            ignore_points: true,
            ignore_lines: true,
        },
    )
    .expect("Failed to load .obj file.");
    let mut objects = HitableList::new();
    for m in models {
        let positions = &m.mesh.positions; //points position
        let indices = &m.mesh.indices; //points index (maybe joint)
        let texcoords = &m.mesh.texcoords;
        let texcoord_indices = &m.mesh.texcoord_indices;

        let mut points = Vec::default();
        let mut triangles = HitableList::new();

        for i in (0..positions.len()).step_by(3) {
            points.push(Vect3::new(positions[i], positions[i + 1], positions[i + 2]) * scale);
        }
        for i in (0..indices.len() - indices.len() % 3).step_by(3) {
            let mut uv = [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)];
            if !texcoords.is_empty() {
                for j in 0..3 {
                    let index = texcoord_indices[i + j] as usize;
                    uv[j] = (texcoords[index << 1], texcoords[index << 1 | 1]);
                }
            }
            triangles.add(Box::new(Triangle::new(
                &points[indices[i] as usize],
                &points[indices[i + 1] as usize],
                &points[indices[i + 2] as usize],
                mat.clone(),
                uv[0],
                uv[1],
                uv[2],
            )));
        }
        objects.add(Box::new(BvhNode::new(triangles, 0., 1.)));
    }
    objects
}

pub fn load_pro(project_name: &str, scale: Vect3, default_color: &Vect3) -> HitableList {
    // must have a .mtl file
    let path_prefix = format!("obj/{}/", project_name);
    let pathname = format!("{}{}.obj", path_prefix, project_name);
    let (models, materials) = tobj::load_obj(
        pathname.clone(),
        &LoadOptions {
            single_index: false,
            triangulate: true,
            ignore_points: true,
            ignore_lines: true,
        },
    )
    .expect(&format!("Failed to load OBJ file {}", pathname.clone()));

    // read mtl texture
    let materials = materials.expect(&format!("Failed to load MTL file {}", pathname));
    let mut textures: Vec<ImageTexture> = Vec::new();
    for mtl in materials {
        if let Some(texture_name) = mtl.diffuse_texture {
            let pathname = format!("{}{}", path_prefix, texture_name);
            let tex = ImageTexture::new(&pathname);
            textures.push(tex);
        } else {
            textures.push(ImageTexture::default());
        }
    }
    let default_mtl = Lambertian::new1(default_color);

    let mut objects = HitableList::new();
    for m in models {
        let positions = &m.mesh.positions; //points position
        let indices = &m.mesh.indices; //points index (maybe joint)
        let texcoords = &m.mesh.texcoords;
        let texcoord_indices = &m.mesh.texcoord_indices;
        let mut points = Vec::new();
        let mut triangles = HitableList::new();
        for i in (0..positions.len()).step_by(3) {
            points.push(Vect3::new(positions[i], positions[i + 1], positions[i + 2]) * scale);
        }

        let tex = &textures[m.mesh.material_id.unwrap()];
        let default = tex.empty();
        let mtl = Lambertian::new2(tex.clone());

        for i in (0..indices.len() - indices.len() % 3).step_by(3) {
            let mut uv = [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)];
            if !texcoords.is_empty() {
                for j in 0..3 {
                    let index = texcoord_indices[i + j] as usize;
                    uv[j] = (texcoords[index << 1], texcoords[index << 1 | 1]);
                }
            }
            if default {
                triangles.add(Box::new(Triangle::new(
                    &points[indices[i] as usize],
                    &points[indices[i + 1] as usize],
                    &points[indices[i + 2] as usize],
                    default_mtl.clone(),
                    uv[0],
                    uv[1],
                    uv[2],
                )));
            } else {
                triangles.add(Box::new(Triangle::new(
                    &points[indices[i] as usize],
                    &points[indices[i + 1] as usize],
                    &points[indices[i + 2] as usize],
                    mtl.clone(),
                    uv[0],
                    uv[1],
                    uv[2],
                )));
            }
        }
        objects.add(Box::new(BvhNode::new(triangles, 0.0, 1.0)));
    }
    objects
}
