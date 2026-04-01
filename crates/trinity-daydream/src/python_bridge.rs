use bevy::prelude::*;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyModule};
use std::sync::Arc;
use avian3d::prelude::*;

/// The component that holds a Python script to be executed every frame.
#[derive(Component)]
pub struct PythonScript {
    pub source: String,
}

/// The Daydream Python scripting plugin.
pub struct PythonPlugin;

impl Plugin for PythonPlugin {
    fn build(&self, app: &mut App) {
        // Prepare PyO3 interpreter gracefully (will use auto-initialize from Cargo.toml if set,
        // but explicit prepare is safe)
        pyo3::prepare_freethreaded_python();
        
        app.add_systems(Update, run_python_scripts);
    }
}

// ─── PyO3 Wrappers for Bevy Data ──────────────────────────────────────────

/// A Python-accessible wrapper for Bevy's Transform (Position and Rotation).
/// This is specifically designed to be XR-ready (6-DoF translation and rotation).
#[pyclass]
#[derive(Clone, Debug)]
pub struct PyTransform {
    // Translation
    #[pyo3(get, set)]
    pub x: f32,
    #[pyo3(get, set)]
    pub y: f32,
    #[pyo3(get, set)]
    pub z: f32,
    
    // Rotation (Euler angles in radians for ease of use in Python)
    #[pyo3(get, set)]
    pub rot_x: f32,
    #[pyo3(get, set)]
    pub rot_y: f32,
    #[pyo3(get, set)]
    pub rot_z: f32,
}

#[pymethods]
impl PyTransform {
    /// Helper to move forward relative to local rotation (useful for XR controllers)
    fn move_forward(&mut self, distance: f32) {
        // Simplified forward vector assuming Y-up rotation
        self.x += self.rot_y.sin() * distance;
        self.z += self.rot_y.cos() * distance;
    }
    
    fn translate(&mut self, dx: f32, dy: f32, dz: f32) {
        self.x += dx;
        self.y += dy;
        self.z += dz;
    }
}

/// A Python-accessible wrapper for Avian3D LinearVelocity.
#[pyclass]
#[derive(Clone, Debug, Default)]
pub struct PyVelocity {
    #[pyo3(get, set)]
    pub x: f32,
    #[pyo3(get, set)]
    pub y: f32,
    #[pyo3(get, set)]
    pub z: f32,
}

#[pymethods]
impl PyVelocity {
    fn add_force(&mut self, fx: f32, fy: f32, fz: f32) {
        self.x += fx;
        self.y += fy;
        self.z += fz;
    }
}

// ─── The Bevy ECS System ──────────────────────────────────────────────────

/// Executes Python scripts attached to entities.
fn run_python_scripts(
    time: Res<Time>,
    // We query optionally for LinearVelocity so scripts can manipulate physics if it exists
    mut query: Query<(Entity, &mut Transform, Option<&mut LinearVelocity>, &PythonScript)>,
) {
    if query.is_empty() { return; }
    let delta = time.delta_secs();

    // Acquire the Global Interpreter Lock once per frame for all scripts
    Python::with_gil(|py| {
        for (entity, mut transform, mut opt_vel, script) in query.iter_mut() {
            // Build the bridge object
            let mut py_trans = PyTransform {
                x: transform.translation.x,
                y: transform.translation.y,
                z: transform.translation.z,
                // Extract rudimentary euler angles (YXZ is common in Bevy for cameras/spatial)
                rot_x: transform.rotation.to_euler(EulerRot::YXZ).1,
                rot_y: transform.rotation.to_euler(EulerRot::YXZ).0,
                rot_z: transform.rotation.to_euler(EulerRot::YXZ).2,
            };

            // Build velocity if it exists
            let mut py_vel = PyVelocity::default();
            if let Some(ref vel) = opt_vel {
                py_vel.x = vel.x;
                py_vel.y = vel.y;
                py_vel.z = vel.z;
            }

            // Setup a secure, isolated context for each script
            let locals = PyDict::new_bound(py);
            
            // Expose our wrappers to the python script
            locals.set_item("transform", py_trans.clone().into_py(py)).unwrap();
            if opt_vel.is_some() {
                locals.set_item("velocity", py_vel.clone().into_py(py)).unwrap();
            }
            
            // Expose standard temporal data
            locals.set_item("delta_time", delta).unwrap();

            // Evaluate the student's python code
            if let Err(e) = py.run_bound(&script.source, None, Some(&locals)) {
                bevy::log::error!("🐍 Python Error on entity {:?}: {}", entity, e);
                continue;
            }

            // Extract the mutated wrapper back
            if let Ok(mutated) = locals.get_item("transform").unwrap().unwrap().extract::<PyTransform>() {
                // Apply spatial changes back to the immutable Bevy universe (Rust side)
                transform.translation.x = mutated.x;
                transform.translation.y = mutated.y;
                transform.translation.z = mutated.z;
                
                // Keep the rotation update simple for now (XR ready)
                transform.rotation = Quat::from_euler(EulerRot::YXZ, mutated.rot_y, mutated.rot_x, mutated.rot_z);
            }

            // Extract mutated velocity back if the entity has physics
            if let Some(mut vel) = opt_vel {
                if let Ok(mutated_vel) = locals.get_item("velocity").unwrap().unwrap().extract::<PyVelocity>() {
                    vel.x = mutated_vel.x;
                    vel.y = mutated_vel.y;
                    vel.z = mutated_vel.z;
                }
            }
        }
    });
}

// ─── Tests ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_api_execution() {
        // Prepare the python interpreter explicitly for the test thread
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let locals = PyDict::new_bound(py);
            
            // Build the bridge object
            let pt = PyTransform { x: 0.0, y: 0.0, z: 0.0, rot_x: 0.0, rot_y: 0.0, rot_z: 0.0 };
            locals.set_item("transform", pt.into_py(py)).unwrap();
            
            // Provide physics context
            let pv = PyVelocity { x: 0.0, y: 0.0, z: 0.0 };
            locals.set_item("velocity", pv.into_py(py)).unwrap();
            
            // Provide temporal context
            locals.set_item("delta_time", 0.5_f32).unwrap();
            
            // Student's python script simulating the Steam Hook logic
            let script = "transform.y += 10.0 * delta_time\nvelocity.y += 50.0";
            py.run_bound(script, None, Some(&locals)).expect("Valid python script failed");
            
            // Extract and assert mutation over 6-DoF XR translation
            let mutated_trans: PyTransform = locals.get_item("transform").unwrap().unwrap().extract().unwrap();
            assert_eq!(mutated_trans.y, 5.0); // 10.0 * 0.5
            
            let mutated_vel: PyVelocity = locals.get_item("velocity").unwrap().unwrap().extract().unwrap();
            assert_eq!(mutated_vel.y, 50.0);
        });
    }
}
