use cgmath::Vector3;

pub struct ParticleState {
    pub variant: Particles,
    pub position: Vector3<f64>,
    pub velocity: Vector3<f64>,
    pub mass: f64,
    pub charge: f64
}

impl ParticleState {
    pub fn set_position(&mut self, pos: Vector3<f64>) {
        self.position = pos;
    }

    pub fn set_velocity(&mut self, velo: Vector3<f64>) {
        self.velocity = velo;
    }
}


pub enum Particles {
    Electron,
    Proton,
    Positron,
    Neutron,
    Test
}

impl Particles {
    pub fn AgentFromParticle(self) -> ParticleState {
        use Particles::*;
        match (self) {
            Electron => {
                ParticleState {
                    variant: self,
                    position: Vector3::new(0f64, 0f64, 0f64),
                    velocity: Vector3::new(0f64, 0f64, 0f64),
                    mass: 9.1e-31f64, // kg 
                    charge: 1.6e-19 // coulboms                    
                }
            },

            _ => {
                ParticleState {
                    variant: self,
                    position: Vector3::new(0f64, 0f64, 0f64),
                    velocity: Vector3::new(0f64, 0f64, 0f64),
                    mass: 1f64, // units
                    charge: -1f64 // units                    
                }                
            }

        }
    }
}