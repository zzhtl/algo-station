use sqlx::sqlite::SqlitePoolOptions;

#[tokio::test]
async fn learning_and_judge_tables_are_migrated() {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::migrate::Migrator::new(std::path::Path::new("./migrations"))
        .await
        .unwrap()
        .run(&pool)
        .await
        .unwrap();

    let names: Vec<String> = sqlx::query_scalar(
        "SELECT name FROM sqlite_master WHERE type = 'table' ORDER BY name",
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    for expected in [
        "daily_plan_overrides",
        "exercise_drafts",
        "judge_workers",
        "learning_events",
        "lesson_progress",
        "quiz_attempts",
        "review_attempts",
        "review_schedules",
        "submissions",
    ] {
        assert!(names.iter().any(|name| name == expected), "missing {expected}");
    }
}
