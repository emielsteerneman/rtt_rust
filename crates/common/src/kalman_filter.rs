/**
 *          https://www.bzarg.com/p/how-a-kalman-filter-works-in-pictures/
 */
use nalgebra::{SMatrix, SVector};

#[allow(dead_code, non_snake_case)]
pub struct KalmanFilter<const S: usize, const O: usize> {
    pub X: SVector<f32, S>,    // State of the system
    pub P: SMatrix<f32, S, S>, // covariance matrix of the system (how sure are we of the state?)

    pub F: SMatrix<f32, S, S>, // Forward model/state update matrix. Essentially a linear model of what we predict the next state will be
    pub H: SMatrix<f32, O, S>, // Observation model / states how we can interpret observation as our state
    pub Q: SMatrix<f32, S, S>, // Covariance of the process noise. (Amount of "Random Forces" we can expect in the process)
    pub R: SMatrix<f32, O, O>, // Observation Noise Covariance. Keeps track of how noisy the observations are.

    // These are only really used in extended Kalman Filters or when we add control input.
    pub B: SMatrix<f32, S, S>,  // State transition jacobian
    pub y: SVector<f32, O>, // Innovation. Not strictly necessary to store but often used to measure performance
}

#[allow(non_snake_case)]
impl<const S: usize, const O: usize> KalmanFilter<S, O> {
    pub fn new(x: SVector<f32, S>, p: SMatrix<f32, S, S>) -> KalmanFilter<S, O> {
        Self {
            X: x,
            P: p,

            F: SMatrix::identity(),
            H: SMatrix::identity(),
            Q: SMatrix::zeros(),
            R: SMatrix::zeros(),

            B: SMatrix::identity(),
            y: SVector::zeros(),
        }
    }

    pub fn predict(&mut self) {
        self.X = self.F * self.X;
        self.P = self.F * self.P * self.F.transpose() + self.Q;
    }

    pub fn update(&mut self, z: SVector<f32, O>) {
        // Compute innovation (error between measurement and state)
        self.y = z - self.H * self.X;
        // Variance of innovation. Named S_ because S is already in use
        let S_ = self.H * self.P * self.H.transpose() + self.R;
        // Compute kalman gain. For small matrices, Eigen's inverse function is most efficient
        let K = self.P * self.H.transpose() * S_.try_inverse().unwrap();
        // Update state with prediction
        self.X = self.X + K * self.y;
        // Update covariance
        self.P -= K * self.H * self.P;
    }

    pub fn covariance(&self) -> &SMatrix<f32, S, S> {
        &self.P
    }

    pub fn state(&self) -> &SVector<f32, S> {
        &self.X
    }

    pub fn set_state(&mut self, x: SVector<f32, S>) {
        self.X = x;
    }

    pub fn set_covariance(&mut self, p: SMatrix<f32, S, S>) {
        self.P = p;
    }

    pub fn modify_state(&mut self, index: usize, x: f32) {
        // TODO What if index is out of bounds?
        self.X[index] = x;
    }
}

impl<const S: usize, const O: usize> Default for KalmanFilter<S, O> {
    fn default() -> Self {
        Self {
            X: SVector::zeros(),
            P: SMatrix::identity(),

            F: SMatrix::identity(),
            H: SMatrix::identity(),
            Q: SMatrix::identity(),
            R: SMatrix::identity(),

            B: SMatrix::identity(),
            y: SVector::zeros(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_test(){
        let mut filter = KalmanFilter::<1, 1>::new(
            SVector::from_vec(vec![60.0]),  // Initial guess
            SMatrix::from_vec(vec![225.0]), // Variance
        );

        filter.B[0] = 1.0;
        // filter.u[0] = 0.0; ?????
        filter.F[0] = 0.0;
        filter.Q[0] = 0.0;
        filter.H[0] = 1.0;
        filter.R[0] = 25.0;

        assert!(filter.state()[0] == 60.0);
        
    }
}