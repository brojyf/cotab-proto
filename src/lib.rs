//! Versioned gRPC contracts shared by the platform services.

pub mod cotab {
    pub mod v1 {
        tonic::include_proto!("cotab.v1");
    }
}

pub mod notify {
    pub mod v1 {
        tonic::include_proto!("notify.v1");
    }
}

pub mod rusti2 {
    pub mod v1 {
        tonic::include_proto!("rusti2.v1");
    }
}
