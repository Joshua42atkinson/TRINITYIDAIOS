// hand_tracking.rs — Hand Tracking for XR
// Uses OpenXR XR_EXT_hand_tracking to detect hand positions.
// Generic: tracks index fingertips and wrists for pointer interaction.

use bevy::prelude::*;

#[cfg(feature = "xr")]
use bevy_mod_openxr::hands::HandBone;

pub struct HandTrackingPlugin;

impl Plugin for HandTrackingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HandTrackingState::default())
            .add_systems(Update, update_hand_tracking);
    }
}

#[derive(Resource, Default)]
pub struct HandTrackingState {
    pub left_index_tip: Option<Vec3>,
    pub right_index_tip: Option<Vec3>,
    pub left_wrist: Option<Vec3>,
    pub right_wrist: Option<Vec3>,
}

#[derive(Component)]
pub struct HandJointMarker {
    pub hand: Handedness,
    pub joint: HandJoint,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Handedness {
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq)]
pub enum HandJoint {
    Wrist,
    ThumbTip,
    IndexTip,
    MiddleTip,
    RingTip,
    PinkyTip,
    IndexProximal,
}

#[cfg(feature = "xr")]
fn update_hand_tracking(
    bone_query: Query<(&HandBone, &Transform)>,
    mut state: ResMut<HandTrackingState>,
) {
    state.left_index_tip = None;
    state.right_index_tip = None;
    state.left_wrist = None;
    state.right_wrist = None;

    for (bone, transform) in bone_query.iter() {
        let pos = transform.translation;
        match bone {
            HandBone::Left => {
                state.left_wrist = Some(pos);
            }
            HandBone::Right => {
                state.right_wrist = Some(pos);
            }
            _ => {}
        }
    }
}

#[cfg(not(feature = "xr"))]
fn update_hand_tracking(
    time: Res<Time>,
    mut state: ResMut<HandTrackingState>,
) {
    let t = time.elapsed_secs();
    let left_pos = Vec3::new(
        (t * 0.3).sin() * 0.3,
        1.0,
        -0.3 + (t * 0.2).cos() * 0.1,
    );
    let right_pos = Vec3::new(
        -0.2 + (t * 0.5).sin() * 0.1,
        1.0,
        -0.1,
    );

    state.left_index_tip = Some(left_pos);
    state.right_index_tip = Some(right_pos);
    state.left_wrist = Some(left_pos + Vec3::new(0.0, -0.05, 0.0));
    state.right_wrist = Some(right_pos + Vec3::new(0.0, -0.05, 0.0));
}
