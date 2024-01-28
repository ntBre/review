use std::{error::Error, fs::read_to_string, path::Path, str::FromStr};

use donkey::{colors::color, vector3, Window};
use raylib_sys::{Camera3D, Vector3};

const NATOMS: usize = 13;

const SYMS: [&str; NATOMS] = [
    "X", "H", "He", "Li", "Be", "B", "C", "N", "O", "F", "Ne", "Na", "Mg",
];

const COLORS: [raylib_sys::Color; NATOMS] = [
    donkey::colors::BLACK, // X
    donkey::colors::WHITE, // H
    donkey::colors::BLACK, // He
    donkey::colors::BLACK, // Li
    donkey::colors::BLACK, // Be
    donkey::colors::BLACK, // B
    donkey::colors::BLACK, // C
    donkey::colors::BLUE,  // N
    donkey::colors::RED,   // O
    donkey::colors::BLACK, // F
    donkey::colors::BLACK, // Ne
    donkey::colors::BLACK, // Na
    donkey::colors::BLACK, // Mg
];

const RADII: [f32; NATOMS] = [
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
];

struct Atom {
    x: f32,
    y: f32,
    z: f32,
    w: u8,
}

impl Atom {
    fn as_vec(&self) -> Vector3 {
        vector3!(self.x, self.y, self.z)
    }
}

impl FromStr for Atom {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sp: Vec<_> = s.split_ascii_whitespace().collect();
        if sp.len() != 4 {
            Err("invalid line length for Atom")?;
        }
        Ok(Atom {
            w: SYMS.iter().position(|&s| s == sp[0]).expect("unknown atom")
                as u8,
            x: sp[1].parse()?,
            y: sp[2].parse()?,
            z: sp[3].parse()?,
        })
    }
}

struct Molecule {
    atoms: Vec<Atom>,
    bonds: Vec<(usize, usize)>,
}

fn load_xyz(path: impl AsRef<Path>) -> Molecule {
    let s = read_to_string(path).unwrap();
    let mut lines = s.lines().enumerate();
    let mut atoms = Vec::new();
    while let Some((i, line)) = lines.next() {
        if i == 0 && line.len() == 1 {
            lines.next(); // discard comment
            continue;
        }
        atoms.push(line.parse().unwrap());
    }

    let mut bonds = Vec::new();
    for i in 0..atoms.len() {
        for j in i + 1..atoms.len() {
            let Atom { x: xi, y: yi, z: zi, .. } = atoms[i];
            let Atom { x: xj, y: yj, z: zj, .. } = atoms[j];
            let dist =
                ((xi - xj).powi(2) + (yi - yj).powi(2) + (zi - zj).powi(2))
                    .sqrt();
            if dist < 3.0 {
                bonds.push((i, j));
            }
        }
    }

    Molecule { atoms, bonds }
}

fn make_window() -> Window {
    let width = 800;
    let height = 600;
    let title = "review";
    Window::init(width, height, title)
}

fn main() {
    let win = make_window();
    win.set_target_fps(30);
    let mut camera = Camera3D {
        position: vector3!(0.0, 0.0, -5.0),
        target: vector3!(0.0, 0.0, 0.0),
        up: vector3!(0.0, 1.0, 0.0),
        fovy: 90.0,
        projection: raylib_sys::CameraProjection_CAMERA_PERSPECTIVE as i32,
    };
    let background = color(0x383838AA);

    let mol = load_xyz("testfiles/acetaldehyde.xyz");

    while !win.should_close() {
        win.begin_drawing();
        win.clear_background(background);
        win.begin_mode3d(camera);

        win.update_camera(&mut camera, donkey::CameraMode::ThirdPerson);

        for atom in &mol.atoms {
            win.draw_sphere(
                atom.as_vec(),
                RADII[atom.w as usize] / 2.0,
                COLORS[atom.w as usize],
            );
        }

        for (i, j) in &mol.bonds {
            win.draw_cylinder(
                mol.atoms[*i].as_vec(),
                mol.atoms[*j].as_vec(),
                0.1,
                donkey::colors::LIGHTGRAY,
            );
        }

        win.end_mode3d();
        win.end_drawing();
    }
}
