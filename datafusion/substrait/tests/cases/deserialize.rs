#[cfg(test)]
mod tests {
    use base64::{engine::general_purpose, Engine as _};
    use datafusion::prelude::{CsvReadOptions, SessionContext};
    use datafusion_substrait::logical_plan::consumer::from_substrait_plan;
    use datafusion_substrait::serializer::{deserialize_bytes};

    // this test case came from https://github.com/apache/datafusion/issues/10412
    #[tokio::test]
    async fn test_typed_not_from_substrait() -> datafusion::common::Result<()> {
        // let protobuf = general_purpose::STANDARD.decode("ChsIARIXL2Z1bmN0aW9uc19ib29sZWFuLnlhbWwSDhoMCAEaCG5vdDpib29sGkcSRQo7OjkKBRIDCgEBEhoKGAoCCgASDQoBRBIICgQKAhABGAI6AwoBVBoUGhIaBAoCEAEiChoIEgYKAhIAIgASBkVYUFIkMA==").unwrap();
        let protobuf = general_purpose::STANDARD.decode("ChsIAhIXL2Z1bmN0aW9uc19ib29sZWFuLnlhbWwKHggBEhovZnVuY3Rpb25zX2NvbXBhcmlzb24ueWFtbBITGhEIARoNZXF1YWw6YW55X2FueRIQGg4IAhABGghub3Q6Ym9vbBpvEm0KYzphCgUSAwoBARJAEj4KAgoAEhoKGAoCCgASDQoBRBIICgQKAhABGAI6AwoBVBocGhoaBAoCEAEiChoIEgYKAhIAIgAiBhoECgIIARoWGhQIARoECgIQASIKGggSBgoCEgAiABIGRVhQUiQw").unwrap();
        let ctx = SessionContext::new();
        ctx.register_csv("T", "tests/testdata/data.csv", CsvReadOptions::new())
            .await?;
        let plan = deserialize_bytes(protobuf).await?;
        let _ = from_substrait_plan(&ctx, plan.as_ref()).await?;
        Ok(())
    }
}