// 100,000 iterations of an electron moving through a magnetic field

extern crate jsonperftest;
use jsonperftest::rusty;
use cgmath::Vector3;

#[macro_use]
extern crate timeit;


pub fn main() {
    timeit! ({
        let mut e = rusty::Particles::Test.AgentFromParticle();

        // Set up an electron with motion orthogonal to the magnetic field
        e.set_velocity(Vector3::new( 0f64, 0.01f64, 0f64));

        let b = Vector3::new(0f64, 0f64, 1.0); // teslas

        let dt = 1e-6f64;

        // println!("Total Velocity: {}", e.velocity.x.powi(2) + e.velocity.y.powi(2) + e.velocity.z.powi(2));
        for _ in 0..1000000 {
            // f = q v x b
            // f = ma
            // a = (qv x b)/m
            // v += a * dt
            // v += (qv x b)*dt/m

            e.set_velocity(
                e.velocity + ( ( (e.charge/e.mass) * ( e.velocity.cross(b) ) ) ) * dt
            );   

            e.set_position(
                e.position + (e.velocity * dt)
            );
        }

        // println!("Total Velocity: {}", e.velocity.x.powi(2) + e.velocity.y.powi(2) + e.velocity.z.powi(2));
        // println!("Final position of the electron {},{},{}", e.position.x, e.position.y, e.position.z);


    });

}