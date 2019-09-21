// The same particle simulation code but where the intermediate representation is the serde value
extern crate jsonperftest;
use jsonperftest::serdey::new_electron;
use jsonperftest::serdey;
use cgmath::Vector3;

#[macro_use]
extern crate timeit;

fn main () {
    timeit!({
        // Make a json representation of the electron
        let mut state = new_electron();

        let b = Vector3::new(0f64, 0f64, 1.0); // teslas

        let dt = 1e-6f64;

        let charge = state["charge"].as_f64().unwrap();
        let mass = state["mass"].as_f64().unwrap();
        let qve = (charge/mass) * dt;


        for _ in 0..1000000 {
            // Pull in the x's, y's, and z's for the position
            let x = state["position_x"].as_f64().unwrap();
            let y = state["position_y"].as_f64().unwrap();
            let z = state["position_z"].as_f64().unwrap();

            // And the velocity
            let vx = state["velocity_x"].as_f64().unwrap();
            let vy = state["velocity_y"].as_f64().unwrap();
            let vz = state["velocity_z"].as_f64().unwrap();

            serdey::set_position(&mut state, x + vx*dt, y + vy*dt, z + vz*dt);

            let a = cross(vx, vy, vz, b.x, b.y, b.z);

            serdey::set_velocity(&mut state, vx + a.0*qve, vy + a.1*qve, vz + a.2*qve);
        }
    });

}

fn cross(
    x1:f64, y1:f64, z1: f64,
    x2:f64, y2:f64, z2: f64
    ) -> (f64, f64, f64) {
        (y1 * z2 - z1*y2, z1 * x2 - x1*z2,  x1*y2 - y2*x2)
}