use derive_more::Constructor;
use nalgebra::Vector2;
use protos::roboteam::WorldBall;

#[derive(Constructor, Default)]
pub struct FilteredBall {
    pub position: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub health: f32,
    pub position_uncertainty: f32,
    pub velocity_uncertainty: f32,
}

impl FilteredBall {
    pub fn as_world_ball(self) -> WorldBall {
        let mut ball = WorldBall::default();

        ball.vel = Some(protos::roboteam::Vector2f {
            x: self.velocity.x,
            y: self.velocity.y,
        });

        ball.pos = Some(protos::roboteam::Vector2f {
            x: self.position.x,
            y: self.position.y,
        });

        // TODO correctly set these values. 
        ball.z = 0.021333; // TODO remove this hardcoded value to some config file
        ball.z_vel = 0.0;
        ball.visible = true;
        ball.area = 0;

        ball
    }
}