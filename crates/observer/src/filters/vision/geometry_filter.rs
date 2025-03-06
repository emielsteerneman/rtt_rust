use std::collections::HashMap;

use protos::sslvision::SslGeometryCameraCalibration;
use protos::sslvision::SslGeometryData;

#[derive(Default)]
pub struct GeometryFilter {
    geometry_received: bool,
    combined_geometry: SslGeometryData,
    cameras: HashMap<u32, SslGeometryCameraCalibration>,
}

impl GeometryFilter {
    fn process(&mut self, geometry_data: &SslGeometryData) -> bool {
        println!("Check if this comparison works the way I think it does");
        if *geometry_data == self.combined_geometry {
            return false;
        }

        for camera in &geometry_data.calib {
            self.cameras.insert(camera.camera_id, camera.clone());
        }

        // In our interpreted geometry we save all the latest camera information we received.
        //  This is relevant as camera geometry may be sent from multiple pc's in the case of a lot of camera's
        self.combined_geometry.calib.clear();
        for (_camera_id, camera) in &self.cameras {
            self.combined_geometry.calib.push(camera.clone());
        }

        self.combined_geometry.field = geometry_data.field.clone();

        self.geometry_received = true;

        true
    }

    fn first_geometry_received(&self) -> bool {
        self.geometry_received
    }

    fn get_geometry(&self) -> &SslGeometryData {
        &self.combined_geometry
    }
}
