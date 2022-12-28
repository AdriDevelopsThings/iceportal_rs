use crate::{ICEPortal, bap::BAPServicStatusResponse};

#[test]
fn test_bap() {
    ICEPortal::fetch::<BAPServicStatusResponse>();
}