use donkey::{colors::color, keys::Key, vector3, Window};
use raylib_sys::Camera3D;

fn main() {
    let width = 800;
    let height = 600;
    let title = "review";
    let cube_size = 1.0;
    let camera_speed = cube_size;
    let mut camera = Camera3D {
        position: vector3!(0.0, 0.0, -cube_size),
        target: vector3!(0.0, 0.0, 0.0),
        up: vector3!(0.0, 1.0, 0.0),
        fovy: 90.0,
        projection: raylib_sys::CameraProjection_CAMERA_PERSPECTIVE as i32,
    };
    let win = Window::init(width, height, title);
    let background = color(0x181818AA);
    while !win.should_close() {
        let dt = win.get_frame_time();
        if win.is_key_down(Key::W) {
            camera.position.z += camera_speed * dt;
        }
        if win.is_key_down(Key::S) {
            camera.position.z -= camera_speed * dt;
        }
        if win.is_key_down(Key::D) {
            camera.position.x += camera_speed * dt;
        }
        if win.is_key_down(Key::A) {
            camera.position.x -= camera_speed * dt;
        }
        win.begin_drawing();
        win.clear_background(background);
        win.begin_mode3d(camera);
        win.draw_sphere(
            vector3!(0.0, 0.0, 0.0),
            cube_size / 2.0,
            0xff0000ff,
        );
        win.end_mode3d();
        win.end_drawing();
    }
}
