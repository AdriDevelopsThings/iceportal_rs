use crate::ICEPortal;

#[tokio::test]
async fn test_bap() {
    ICEPortal::fetch_bap().await.unwrap();
}