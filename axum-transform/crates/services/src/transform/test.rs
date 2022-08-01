use super::*;
use rstest::*;
use speculoos::assert_that;

#[fixture]
pub fn service() -> StdTransform {
    StdTransform::new()
}

#[rstest]
#[case::empty("", "")]
#[case::hello("Hello", "HELLO")]
#[trace]
#[tokio::test]
pub async fn std_capitalizes_sentences(
    service: StdTransform,
    #[case] data: String,
    #[case] expected: String,
) {
    let transformation = Transformation::Capitalize;
    let request = TransformRequest {
        transformation,
        data,
    };
    let response = service.transform(request).await;
    let obtained = response.data;

    assert_that!(obtained).is_equal_to(expected)
}
