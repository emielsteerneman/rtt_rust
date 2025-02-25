use std::collections::HashMap;

use protos::messages::SslGeometryData;
use protos::messages::SslGeometryCameraCalibration;

struct GeometryFilter {
    last_geometry_string: String,
    combined_geometry: SslGeometryData,
    cameras: HashMap<u32, SslGeometryCameraCalibration>,
}

impl GeometryFilter {
    fn process(&mut self, geometry_data: &SslGeometryData) -> bool {
        if *geometry_data == self.combined_geometry {
            return false;
        }

        for camera in &geometry_data.calib {
            self.cameras.insert(camera.camera_id, camera.clone());
        }

        self.combined_geometry.calib.clear();

        

        true
    }
}