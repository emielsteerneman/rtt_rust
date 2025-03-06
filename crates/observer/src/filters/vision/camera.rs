//! # Note on implementation
//! All of the following functions are just Nalgebra implementations of the logic in SSL-Vision, so nothing here is really
//! 'created' by us. If you have problems understanding it I recommend looking further at camera_calibration.h in ssl-vision
//! or to send me (Rolf) a message so I can help explaining it. Note the SSL-vision code may change, so compatibility can
//! break here
//! * 1
//! * 2
//! * 3
//! * 4
//! * 5
//! * 6
use derive_more::Constructor;
use nalgebra::{Vector2, Vector3, Quaternion};

use protos::sslvision::SslGeometryCameraCalibration;


#[derive(Constructor, Default)]
struct Camera {
    id: u32,
    position: Vector3<f32>,
    translation: Vector3<f32>,
    rotation: Quaternion<f32>,
    principal_point: Vector2<f32>,
    focal_length: f32,
    distortion: f32,
    image_width: u32,
    image_height: u32,
}

impl Camera {
    /// Construct a camera from the calibration data received in a ssl-vision message
    pub fn from_camera_calibration(camera_calibration: &SslGeometryCameraCalibration) -> Self {
        todo!()
    }

    /// Returns the 3d position of the camera in metres [m]
    pub fn get_position(&self) -> Vector3<f32> {
        self.position
    }
    /// Returns the quaternion corresponding to the rotation matrix of the estimated camera rotation
    pub fn get_orientation(&self) -> Quaternion<f32> {
        self.rotation
    }
    /// Returns the id of the camera
    pub fn get_id(&self) -> u32 {
        self.id
    }

    /// All vectors are in meters
    /// 
    /// # Parameters
    /// - `object_position` 3d position of object to be project to plane
    /// - `plane_height` height of the plane
    /// 
    /// # Returns
    /// The 2d vector in meters on the plane
    pub fn linear_project_to_horizontal_plane(&self, object_position: &Vector3<f32>, plane_height: f32) -> Vector2<f32> {
        todo!()
    }

    /// Checks if the position is visible
    /// 
    /// The margin_factor is subtracted from the image boundaries in pixels on each side. It is then checked if the 
    /// coordinate produced falls within the acceptable range. E.g. if marginFactor is 0.1 and the image is 1280x1024,
    /// 128 pixels are substracted from each side and only the inner region is considered visible.
    /// 
    /// # Parameters
    /// - `field_point_mm` 3d position of the object to be checked **in millimeters** 
    /// - `margin_factor` the factor to be subtracted from the image boundaries 
    /// 
    /// # Returns
    /// True if the position is visible, false otherwise. This function also returns true if either the width or height
    /// of the image is unknown.
    pub fn is_position_visible(&self, field_point_mm: &Vector3<f32>, margin_factor: f32) -> bool {
        todo!()
    }

    /// Projects a 3d point in the field to a 2d point in the image. See [note](index.html#note-on-implementation)
    /// 
    /// # Parameters
    /// - `field_point_mm` 3d position of the object to be projected **in millimeters**
    /// 
    /// # Returns
    /// the pixel coordinates of the 3d point that was projected
    pub fn field_to_image(&self, field_point_mm: &Vector3<f32>) -> Vector2<f32> {
        todo!()
    }


}