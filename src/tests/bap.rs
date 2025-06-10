use crate::{tests::prepare::prepare_tests, ICEPortal};

#[tokio::test]
async fn test_bap() {
    prepare_tests();
    ICEPortal::fetch_bap().await.unwrap();
}
