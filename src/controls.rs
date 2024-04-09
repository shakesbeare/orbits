use crate::keyboard::get_scan_code;
use bevy::prelude::*;

#[derive(Component)]
pub struct MoveVector(pub Vec3);

#[derive(Component)]
pub enum ControlScheme {
    Keyboard {
        forward: ScanCode,
        backward: ScanCode,
        left: ScanCode,
        right: ScanCode,
        zoom_in: ScanCode,
        zoom_out: ScanCode,
    },
}

impl ControlScheme {
    pub fn wasd() -> Self {
        ControlScheme::Keyboard {
            forward: ScanCode(get_scan_code("W")),
            backward: ScanCode(get_scan_code("S")),
            left: ScanCode(get_scan_code("A")),
            right: ScanCode(get_scan_code("D")),
            zoom_in: ScanCode(get_scan_code("E")),
            zoom_out: ScanCode(get_scan_code("Q")),
        }
    }

    pub fn arrow() -> Self {
        ControlScheme::Keyboard {
            forward: ScanCode(get_scan_code("Up")),
            backward: ScanCode(get_scan_code("Down")),
            left: ScanCode(get_scan_code("Left")),
            right: ScanCode(get_scan_code("Right")),
            zoom_in: ScanCode(get_scan_code("PageUp")),
            zoom_out: ScanCode(get_scan_code("PageDown")),
        }
    }
}

pub struct ControlState {
    pub forward: isize,
    pub backward: isize,
    pub left: isize,
    pub right: isize,
    pub zoom_in: isize,
    pub zoom_out: isize,
}

pub fn print_scan_codes(
    mut keys: EventReader<bevy::input::keyboard::KeyboardInput>,
) {
    for ev in keys.read() {
        let sc = format!("{:#02x}", ev.scan_code);
        dbg!(sc);
    }
}

pub fn handle_camera_move_input(
    keys: Res<Input<ScanCode>>,
    mut query: Query<(&mut MoveVector, &ControlScheme)>,
) {
    for (mut mv, cs) in query.iter_mut() {
        let input = match cs {
            ControlScheme::Keyboard {
                forward,
                backward,
                left,
                right,
                zoom_in,
                zoom_out,
            } => ControlState {
                backward: if keys.pressed(*backward) { 1 } else { 0 },
                forward: if keys.pressed(*forward) { 1 } else { 0 },
                left: if keys.pressed(*left) { 1 } else { 0 },
                right: if keys.pressed(*right) { 1 } else { 0 },
                zoom_in: if keys.pressed(*zoom_in) { 1 } else { 0 },
                zoom_out: if keys.pressed(*zoom_out) { 1 } else { 0 },
            },
        };

        mv.0 = Vec3::new(
            (input.right - input.left) as f32,
            (input.forward - input.backward) as f32,
            (input.zoom_out - input.zoom_in) as f32,
        );
    }
}

// Handles user control over the timewarp level
// and updates the timewarp factor accordingly
pub fn handle_timewarp_input(
    keys: Res<Input<ScanCode>>,
    mut timewarp_level: ResMut<crate::TimewarpLevel>,
    mut timewarp_factor: ResMut<crate::TimewarpFactor>,
) {
    let scales = [1, 5, 10, 100, 1000, 10_000, 100_000];

    // If period key is pressed and timewarp level is not max, 
    // increase timewarp level
    if keys.just_pressed(ScanCode(get_scan_code(".")))
        && timewarp_level.0 < scales.len() - 1
    {
        timewarp_level.0 += 1;
    // If comma key is pressed and timewarp level is not min, 
    // decrease timewarp level
    } else if keys.just_pressed(ScanCode(get_scan_code(",")))
        && timewarp_level.0 > 0
    {
        timewarp_level.0 -= 1;
    }

    for key in keys.get_just_pressed() {
        if key.0 == get_scan_code(",") || key.0 == get_scan_code(".") {
            timewarp_factor.0 = scales[timewarp_level.0] as f64;
            dbg!(timewarp_factor.0, timewarp_level.0);
        }
    }
}
