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

#[cfg(test)]
mod tests {
    use super::rusti2::v1::StatObjectRequest;
    use prost::Message;

    #[test]
    fn stat_object_wire_contract_matches_go() {
        let request = StatObjectRequest {
            bucket: "media".into(),
            key: "avatar".into(),
        };
        let wire = b"\x0a\x05media\x12\x06avatar";
        assert_eq!(request.encode_to_vec(), wire);
        assert_eq!(StatObjectRequest::decode(wire.as_slice()).unwrap(), request);
    }
}
