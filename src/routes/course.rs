use std::collections::{HashMap, HashSet};

use axum::extract::{Path, Query, State};
use axum::Json;
use chrono::{Duration, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::curriculum::{
    ExerciseDefinition, LessonDefinition, QuizQuestion, StageDefinition, VisualizationDefinition,
};
use crate::error::{ApiError, ApiResult};
use crate::learning::{lesson_is_complete, CompletionEvidence, ReviewRating, ReviewSchedule};
use crate::state::AppState;

#[derive(Debug, Clone, sqlx::FromRow)]
struct ProgressRow {
    lesson_slug: String,
    status: String,
    animation_completed: i64,
    quiz_best_score: i64,
    note: String,
    completed_at: Option<String>,
    updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProgressView {
    lesson_slug: String,
    status: String,
    animation_completed: bool,
    quiz_best_score: u8,
    core_exercise_accepted: bool,
    note: String,
    completed_at: Option<String>,
    updated_at: Option<String>,
}

impl ProgressView {
    fn empty(lesson_slug: &str) -> Self {
        Self {
            lesson_slug: lesson_slug.to_owned(),
            status: "not_started".to_owned(),
            animation_completed: false,
            quiz_best_score: 0,
            core_exercise_accepted: false,
            note: String::new(),
            completed_at: None,
            updated_at: None,
        }
    }
}

#[derive(Serialize)]
pub struct CurriculumSummary {
    stage_count: usize,
    lesson_count: usize,
    exercise_count: usize,
    visualization_count: usize,
    completed_lessons: usize,
    completion_percent: u8,
}

#[derive(Serialize)]
pub struct CurriculumResponse {
    summary: CurriculumSummary,
    stages: Vec<StageView>,
}

#[derive(Serialize)]
pub struct StageView {
    id: String,
    title: String,
    description: String,
    order: u16,
    completed_lessons: usize,
    lessons: Vec<LessonCard>,
}

#[derive(Serialize, Clone)]
pub struct LessonCard {
    slug: String,
    stage_id: String,
    title: String,
    summary: String,
    order: u16,
    estimated_minutes: u16,
    status: String,
    prerequisites_met: bool,
    has_visualization: bool,
    exercise_count: usize,
}

#[derive(Serialize)]
pub struct PublicQuestion {
    id: String,
    prompt: String,
    options: Vec<String>,
}

impl From<&QuizQuestion> for PublicQuestion {
    fn from(question: &QuizQuestion) -> Self {
        Self {
            id: question.id.clone(),
            prompt: question.prompt.clone(),
            options: question.options.clone(),
        }
    }
}

#[derive(Serialize)]
pub struct ExerciseSummary {
    slug: String,
    problem_id: i64,
    title: String,
    difficulty: String,
    summary: String,
    core: bool,
    accepted: bool,
}

#[derive(Serialize)]
pub struct LessonPublic {
    slug: String,
    stage_id: String,
    article_slug: String,
    title: String,
    summary: String,
    order: u16,
    estimated_minutes: u16,
    prerequisites: Vec<String>,
    objectives: Vec<String>,
    quiz: Vec<PublicQuestion>,
}

impl From<&LessonDefinition> for LessonPublic {
    fn from(lesson: &LessonDefinition) -> Self {
        Self {
            slug: lesson.slug.clone(),
            stage_id: lesson.stage_id.clone(),
            article_slug: lesson.article_slug.clone(),
            title: lesson.title.clone(),
            summary: lesson.summary.clone(),
            order: lesson.order,
            estimated_minutes: lesson.estimated_minutes,
            prerequisites: lesson.prerequisites.clone(),
            objectives: lesson.objectives.clone(),
            quiz: lesson.quiz.iter().map(Into::into).collect(),
        }
    }
}

#[derive(Serialize)]
pub struct LessonDetailResponse {
    lesson: LessonPublic,
    progress: ProgressView,
    prerequisites_met: bool,
    visualization: Option<VisualizationDefinition>,
    exercises: Vec<ExerciseSummary>,
}

pub async fn curriculum(State(state): State<AppState>) -> ApiResult<Json<CurriculumResponse>> {
    let progress = progress_map(&state).await?;
    let completed_lessons = progress
        .values()
        .filter(|row| row.status == "completed")
        .count();
    let stages = state
        .catalog
        .stages
        .iter()
        .map(|stage| stage_view(&state, stage, &progress))
        .collect();
    let lesson_count = state.catalog.lessons.len();
    let completion_percent = (completed_lessons * 100)
        .checked_div(lesson_count)
        .unwrap_or(0) as u8;

    Ok(Json(CurriculumResponse {
        summary: CurriculumSummary {
            stage_count: state.catalog.stages.len(),
            lesson_count,
            exercise_count: state.catalog.exercises.len(),
            visualization_count: state.catalog.visualizations.len(),
            completed_lessons,
            completion_percent,
        },
        stages,
    }))
}

fn stage_view(
    state: &AppState,
    stage: &StageDefinition,
    progress: &HashMap<String, ProgressRow>,
) -> StageView {
    let lessons: Vec<LessonCard> = stage
        .lesson_slugs
        .iter()
        .filter_map(|slug| state.catalog.lessons.iter().find(|lesson| &lesson.slug == slug))
        .map(|lesson| lesson_card(lesson, progress))
        .collect();
    StageView {
        id: stage.id.clone(),
        title: stage.title.clone(),
        description: stage.description.clone(),
        order: stage.order,
        completed_lessons: lessons
            .iter()
            .filter(|lesson| lesson.status == "completed")
            .count(),
        lessons,
    }
}

fn lesson_card(
    lesson: &LessonDefinition,
    progress: &HashMap<String, ProgressRow>,
) -> LessonCard {
    LessonCard {
        slug: lesson.slug.clone(),
        stage_id: lesson.stage_id.clone(),
        title: lesson.title.clone(),
        summary: lesson.summary.clone(),
        order: lesson.order,
        estimated_minutes: lesson.estimated_minutes,
        status: progress
            .get(&lesson.slug)
            .map(|row| row.status.clone())
            .unwrap_or_else(|| "not_started".to_owned()),
        prerequisites_met: lesson.prerequisites.iter().all(|slug| {
            progress
                .get(slug)
                .is_some_and(|row| row.status == "completed")
        }),
        has_visualization: lesson.visualization_id.is_some(),
        exercise_count: lesson.exercise_slugs.len(),
    }
}

pub async fn lesson_detail(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> ApiResult<Json<LessonDetailResponse>> {
    let lesson = find_lesson(&state, &slug)?;
    let progress_rows = progress_map(&state).await?;
    let prerequisites_met = lesson.prerequisites.iter().all(|prerequisite| {
        progress_rows
            .get(prerequisite)
            .is_some_and(|row| row.status == "completed")
    });
    let progress = progress_view(&state, lesson).await?;
    let accepted = accepted_exercises(&state, &lesson.exercise_slugs).await?;
    let exercises = lesson
        .exercise_slugs
        .iter()
        .filter_map(|exercise_slug| {
            state
                .catalog
                .exercises
                .iter()
                .find(|exercise| &exercise.slug == exercise_slug)
        })
        .map(|exercise| exercise_summary(exercise, lesson, &accepted))
        .collect();
    let visualization = lesson.visualization_id.as_ref().and_then(|id| {
        state
            .catalog
            .visualizations
            .iter()
            .find(|visualization| &visualization.id == id)
            .cloned()
    });

    Ok(Json(LessonDetailResponse {
        lesson: lesson.into(),
        progress,
        prerequisites_met,
        visualization,
        exercises,
    }))
}

fn exercise_summary(
    exercise: &ExerciseDefinition,
    lesson: &LessonDefinition,
    accepted: &HashSet<String>,
) -> ExerciseSummary {
    ExerciseSummary {
        slug: exercise.slug.clone(),
        problem_id: exercise.problem_id,
        title: exercise.title.clone(),
        difficulty: exercise.difficulty.clone(),
        summary: exercise.summary.clone(),
        core: lesson.core_exercise_slugs.contains(&exercise.slug),
        accepted: accepted.contains(&exercise.slug),
    }
}

#[derive(Deserialize)]
pub struct ProgressInput {
    animation_completed: Option<bool>,
    note: Option<String>,
}

pub async fn patch_progress(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(input): Json<ProgressInput>,
) -> ApiResult<Json<ProgressView>> {
    let lesson = find_lesson(&state, &slug)?;
    let existing = fetch_progress(&state, &slug).await?;
    let animation_completed = input
        .animation_completed
        .unwrap_or_else(|| existing.as_ref().is_some_and(|row| row.animation_completed != 0));
    let note = input
        .note
        .unwrap_or_else(|| existing.as_ref().map(|row| row.note.clone()).unwrap_or_default());
    let best = existing
        .as_ref()
        .map(|row| row.quiz_best_score)
        .unwrap_or(0);
    upsert_progress_fields(&state, &slug, animation_completed, best, &note).await?;
    Ok(Json(derive_lesson_progress(&state, lesson).await?))
}

#[derive(Deserialize)]
pub struct QuizAttemptInput {
    answers: Vec<usize>,
}

#[derive(Serialize)]
pub struct QuizCorrection {
    question_id: String,
    selected_index: usize,
    correct_index: usize,
    correct: bool,
    explanation: String,
}

#[derive(Serialize)]
pub struct QuizAttemptResponse {
    score: u8,
    passed: bool,
    corrections: Vec<QuizCorrection>,
    progress: ProgressView,
}

pub async fn quiz_attempt(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(input): Json<QuizAttemptInput>,
) -> ApiResult<Json<QuizAttemptResponse>> {
    let lesson = find_lesson(&state, &slug)?;
    if input.answers.len() != lesson.quiz.len() {
        return Err(ApiError::BadRequest(format!(
            "answers must contain exactly {} items",
            lesson.quiz.len()
        )));
    }
    let correct = lesson
        .quiz
        .iter()
        .zip(&input.answers)
        .filter(|(question, selected)| question.correct_index == **selected)
        .count();
    let score = ((correct * 100) / lesson.quiz.len()) as u8;
    let answers_json = serde_json::to_string(&input.answers)
        .map_err(|error| ApiError::BadRequest(error.to_string()))?;
    sqlx::query(
        "INSERT INTO quiz_attempts (lesson_slug, score, total, answers) VALUES (?, ?, ?, ?)",
    )
    .bind(&slug)
    .bind(i64::from(score))
    .bind(lesson.quiz.len() as i64)
    .bind(answers_json)
    .execute(&state.pool)
    .await?;

    let existing = fetch_progress(&state, &slug).await?;
    let best = existing
        .as_ref()
        .map(|row| row.quiz_best_score)
        .unwrap_or(0)
        .max(i64::from(score));
    upsert_progress_fields(
        &state,
        &slug,
        existing.as_ref().is_some_and(|row| row.animation_completed != 0),
        best,
        &existing.map(|row| row.note).unwrap_or_default(),
    )
    .await?;
    record_event(&state, "quiz_attempted", Some(&slug), None).await?;
    let progress = derive_lesson_progress(&state, lesson).await?;
    let corrections = lesson
        .quiz
        .iter()
        .zip(input.answers)
        .map(|(question, selected_index)| QuizCorrection {
            question_id: question.id.clone(),
            selected_index,
            correct_index: question.correct_index,
            correct: selected_index == question.correct_index,
            explanation: question.explanation.clone(),
        })
        .collect();

    Ok(Json(QuizAttemptResponse {
        score,
        passed: score >= 80,
        corrections,
        progress,
    }))
}

#[derive(Deserialize)]
pub struct PlanQuery {
    date: Option<String>,
}

#[derive(Serialize)]
pub struct DailyPlan {
    date: String,
    target_minutes: u16,
    estimated_minutes: u16,
    lessons: Vec<LessonCard>,
}

pub async fn daily_plan(
    State(state): State<AppState>,
    Query(query): Query<PlanQuery>,
) -> ApiResult<Json<DailyPlan>> {
    let date = validated_date(query.date)?;
    Ok(Json(build_daily_plan(&state, &date).await?))
}

#[derive(Deserialize)]
pub struct DailyPlanInput {
    date: Option<String>,
    target_minutes: u16,
}

pub async fn put_daily_plan(
    State(state): State<AppState>,
    Json(input): Json<DailyPlanInput>,
) -> ApiResult<Json<DailyPlan>> {
    if !(10..=240).contains(&input.target_minutes) {
        return Err(ApiError::BadRequest(
            "target_minutes must be between 10 and 240".to_owned(),
        ));
    }
    let date = validated_date(input.date)?;
    sqlx::query(
        "INSERT INTO daily_plan_overrides (plan_date, target_minutes, updated_at)
         VALUES (?, ?, datetime('now'))
         ON CONFLICT(plan_date) DO UPDATE SET
             target_minutes=excluded.target_minutes, updated_at=excluded.updated_at",
    )
    .bind(&date)
    .bind(i64::from(input.target_minutes))
    .execute(&state.pool)
    .await?;
    Ok(Json(build_daily_plan(&state, &date).await?))
}

async fn build_daily_plan(state: &AppState, date: &str) -> ApiResult<DailyPlan> {
    let target: i64 = sqlx::query_scalar(
        "SELECT target_minutes FROM daily_plan_overrides WHERE plan_date = ?",
    )
    .bind(date)
    .fetch_optional(&state.pool)
    .await?
    .unwrap_or(60);
    let progress = progress_map(state).await?;
    let mut estimated = 0u16;
    let mut lessons = Vec::new();
    for lesson in &state.catalog.lessons {
        if progress
            .get(&lesson.slug)
            .is_some_and(|row| row.status == "completed")
        {
            continue;
        }
        lessons.push(lesson_card(lesson, &progress));
        estimated = estimated.saturating_add(lesson.estimated_minutes);
        if estimated >= target as u16 {
            break;
        }
    }
    Ok(DailyPlan {
        date: date.to_owned(),
        target_minutes: target as u16,
        estimated_minutes: estimated,
        lessons,
    })
}

#[derive(Serialize)]
pub struct DashboardResponse {
    completed_lessons: usize,
    total_lessons: usize,
    completion_percent: u8,
    streak_days: usize,
    due_reviews: i64,
    next_lesson: Option<LessonCard>,
    daily_plan: DailyPlan,
    judge_online: bool,
}

pub async fn dashboard(State(state): State<AppState>) -> ApiResult<Json<DashboardResponse>> {
    let progress = progress_map(&state).await?;
    let completed_lessons = progress
        .values()
        .filter(|row| row.status == "completed")
        .count();
    let total_lessons = state.catalog.lessons.len();
    let next_lesson = state
        .catalog
        .lessons
        .iter()
        .find(|lesson| {
            progress
                .get(&lesson.slug)
                .is_none_or(|row| row.status != "completed")
        })
        .map(|lesson| lesson_card(lesson, &progress));
    let due_reviews: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM review_schedules WHERE due_at <= datetime('now')",
    )
    .fetch_one(&state.pool)
    .await?;
    let judge_online: bool = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM judge_workers
         WHERE status = 'online' AND last_heartbeat_at >= datetime('now', '-30 seconds')",
    )
    .fetch_one(&state.pool)
    .await?
        > 0;
    let today = Utc::now().date_naive().format("%Y-%m-%d").to_string();
    let daily_plan = build_daily_plan(&state, &today).await?;

    Ok(Json(DashboardResponse {
        completed_lessons,
        total_lessons,
        completion_percent: (completed_lessons * 100)
            .checked_div(total_lessons)
            .unwrap_or(0) as u8,
        streak_days: learning_streak(&state).await?,
        due_reviews,
        next_lesson,
        daily_plan,
        judge_online,
    }))
}

#[derive(Serialize, sqlx::FromRow)]
pub struct ReviewItem {
    lesson_slug: String,
    title: String,
    step: i64,
    due_at: String,
    last_rating: Option<String>,
    mastered: bool,
    due: bool,
}

pub async fn reviews(State(state): State<AppState>) -> ApiResult<Json<Vec<ReviewItem>>> {
    #[derive(sqlx::FromRow)]
    struct ScheduleRow {
        lesson_slug: String,
        step: i64,
        due_at: String,
        last_rating: Option<String>,
        mastered: i64,
        due: i64,
    }
    let rows = sqlx::query_as::<_, ScheduleRow>(
        "SELECT lesson_slug, step, due_at, last_rating, mastered,
                due_at <= datetime('now') AS due
         FROM review_schedules ORDER BY due DESC, due_at, lesson_slug",
    )
    .fetch_all(&state.pool)
    .await?;
    Ok(Json(
        rows.into_iter()
            .filter_map(|row| {
                let title = state
                    .catalog
                    .lessons
                    .iter()
                    .find(|lesson| lesson.slug == row.lesson_slug)?
                    .title
                    .clone();
                Some(ReviewItem {
                    lesson_slug: row.lesson_slug,
                    title,
                    step: row.step,
                    due_at: row.due_at,
                    last_rating: row.last_rating,
                    mastered: row.mastered != 0,
                    due: row.due != 0,
                })
            })
            .collect(),
    ))
}

#[derive(Deserialize)]
pub struct ReviewAttemptInput {
    rating: ReviewRating,
}

pub async fn review_attempt(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(input): Json<ReviewAttemptInput>,
) -> ApiResult<Json<ReviewItem>> {
    let lesson = find_lesson(&state, &slug)?;
    let current: (i64, String, Option<String>, i64) = sqlx::query_as(
        "SELECT step, due_at, last_rating, mastered FROM review_schedules WHERE lesson_slug = ?",
    )
    .bind(&slug)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| ApiError::BadRequest("lesson has no review schedule".to_owned()))?;
    let today = Utc::now().timestamp().div_euclid(86_400);
    let next = ReviewSchedule {
        level: current.0 as u8,
        due_day: today,
        mastered: current.3 != 0,
    }
    .after(input.rating, today);
    let next_due = sql_datetime_from_day(next.due_day)?;
    let rating = rating_name(input.rating);
    let mut transaction = state.pool.begin().await?;
    sqlx::query(
        "UPDATE review_schedules SET step = ?, due_at = ?, last_rating = ?, mastered = ?,
                updated_at = datetime('now') WHERE lesson_slug = ?",
    )
    .bind(i64::from(next.level))
    .bind(&next_due)
    .bind(rating)
    .bind(i64::from(next.mastered))
    .bind(&slug)
    .execute(&mut *transaction)
    .await?;
    sqlx::query(
        "INSERT INTO review_attempts
         (lesson_slug, rating, previous_step, next_step, next_due_at)
         VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&slug)
    .bind(rating)
    .bind(current.0)
    .bind(i64::from(next.level))
    .bind(&next_due)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    record_event(&state, "reviewed", Some(&slug), None).await?;

    Ok(Json(ReviewItem {
        lesson_slug: slug,
        title: lesson.title.clone(),
        step: i64::from(next.level),
        due_at: next_due,
        last_rating: Some(rating.to_owned()),
        mastered: next.mastered,
        due: false,
    }))
}

pub async fn derive_lesson_progress(
    state: &AppState,
    lesson: &LessonDefinition,
) -> ApiResult<ProgressView> {
    let existing = fetch_progress(state, &lesson.slug).await?;
    let row = existing.unwrap_or(ProgressRow {
        lesson_slug: lesson.slug.clone(),
        status: "not_started".to_owned(),
        animation_completed: 0,
        quiz_best_score: 0,
        note: String::new(),
        completed_at: None,
        updated_at: String::new(),
    });
    let accepted = accepted_exercises(state, &lesson.core_exercise_slugs).await?;
    let core_exercise_accepted = lesson
        .core_exercise_slugs
        .iter()
        .all(|slug| accepted.contains(slug));
    let complete = row.status == "completed"
        || lesson_is_complete(&CompletionEvidence {
            best_quiz_score: row.quiz_best_score.clamp(0, 100) as u8,
            core_exercise_accepted,
            visualization_required: lesson.visualization_id.is_some(),
            visualization_completed: row.animation_completed != 0,
        });
    let started = row.quiz_best_score > 0
        || row.animation_completed != 0
        || core_exercise_accepted
        || !row.note.is_empty();
    let status = if complete {
        "completed"
    } else if started {
        "in_progress"
    } else {
        "not_started"
    };
    let transitioned = row.status != "completed" && status == "completed";
    let completed_at = if status == "completed" {
        row.completed_at.clone().or_else(|| Some(now_sql()))
    } else {
        None
    };
    sqlx::query(
        "INSERT INTO lesson_progress
         (lesson_slug, status, animation_completed, quiz_best_score, note, completed_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, datetime('now'))
         ON CONFLICT(lesson_slug) DO UPDATE SET
             status=excluded.status,
             animation_completed=excluded.animation_completed,
             quiz_best_score=excluded.quiz_best_score,
             note=excluded.note,
             completed_at=excluded.completed_at,
             updated_at=excluded.updated_at",
    )
    .bind(&lesson.slug)
    .bind(status)
    .bind(row.animation_completed)
    .bind(row.quiz_best_score)
    .bind(&row.note)
    .bind(&completed_at)
    .execute(&state.pool)
    .await?;

    if transitioned {
        let due_at = (Utc::now() + Duration::days(1))
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        sqlx::query(
            "INSERT OR IGNORE INTO review_schedules (lesson_slug, step, due_at) VALUES (?, 0, ?)",
        )
        .bind(&lesson.slug)
        .bind(due_at)
        .execute(&state.pool)
        .await?;
        record_event(state, "lesson_completed", Some(&lesson.slug), None).await?;
    }

    Ok(ProgressView {
        lesson_slug: lesson.slug.clone(),
        status: status.to_owned(),
        animation_completed: row.animation_completed != 0,
        quiz_best_score: row.quiz_best_score.clamp(0, 100) as u8,
        core_exercise_accepted,
        note: row.note,
        completed_at,
        updated_at: Some(now_sql()),
    })
}

async fn progress_view(
    state: &AppState,
    lesson: &LessonDefinition,
) -> ApiResult<ProgressView> {
    let Some(row) = fetch_progress(state, &lesson.slug).await? else {
        return Ok(ProgressView::empty(&lesson.slug));
    };
    let accepted = accepted_exercises(state, &lesson.core_exercise_slugs).await?;
    Ok(ProgressView {
        lesson_slug: row.lesson_slug,
        status: row.status,
        animation_completed: row.animation_completed != 0,
        quiz_best_score: row.quiz_best_score.clamp(0, 100) as u8,
        core_exercise_accepted: lesson
            .core_exercise_slugs
            .iter()
            .all(|slug| accepted.contains(slug)),
        note: row.note,
        completed_at: row.completed_at,
        updated_at: Some(row.updated_at),
    })
}

async fn fetch_progress(state: &AppState, slug: &str) -> ApiResult<Option<ProgressRow>> {
    Ok(sqlx::query_as::<_, ProgressRow>(
        "SELECT lesson_slug, status, animation_completed, quiz_best_score, note,
                completed_at, updated_at FROM lesson_progress WHERE lesson_slug = ?",
    )
    .bind(slug)
    .fetch_optional(&state.pool)
    .await?)
}

async fn upsert_progress_fields(
    state: &AppState,
    slug: &str,
    animation_completed: bool,
    quiz_best_score: i64,
    note: &str,
) -> ApiResult<()> {
    sqlx::query(
        "INSERT INTO lesson_progress
         (lesson_slug, status, animation_completed, quiz_best_score, note, updated_at)
         VALUES (?, 'in_progress', ?, ?, ?, datetime('now'))
         ON CONFLICT(lesson_slug) DO UPDATE SET
             animation_completed=excluded.animation_completed,
             quiz_best_score=MAX(lesson_progress.quiz_best_score, excluded.quiz_best_score),
             note=excluded.note,
             updated_at=excluded.updated_at",
    )
    .bind(slug)
    .bind(i64::from(animation_completed))
    .bind(quiz_best_score)
    .bind(note)
    .execute(&state.pool)
    .await?;
    Ok(())
}

async fn progress_map(state: &AppState) -> ApiResult<HashMap<String, ProgressRow>> {
    let rows = sqlx::query_as::<_, ProgressRow>(
        "SELECT lesson_slug, status, animation_completed, quiz_best_score, note,
                completed_at, updated_at FROM lesson_progress",
    )
    .fetch_all(&state.pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|row| (row.lesson_slug.clone(), row))
        .collect())
}

async fn accepted_exercises(
    state: &AppState,
    exercise_slugs: &[String],
) -> ApiResult<HashSet<String>> {
    let rows: Vec<String> = sqlx::query_scalar(
        "SELECT DISTINCT exercise_slug FROM submissions WHERE status = 'accepted'",
    )
    .fetch_all(&state.pool)
    .await?;
    let requested: HashSet<&str> = exercise_slugs.iter().map(String::as_str).collect();
    Ok(rows
        .into_iter()
        .filter(|slug| requested.contains(slug.as_str()))
        .collect())
}

fn find_lesson<'a>(state: &'a AppState, slug: &str) -> ApiResult<&'a LessonDefinition> {
    state
        .catalog
        .lessons
        .iter()
        .find(|lesson| lesson.slug == slug)
        .ok_or(ApiError::NotFound)
}

fn validated_date(value: Option<String>) -> ApiResult<String> {
    let value = value.unwrap_or_else(|| Utc::now().date_naive().format("%Y-%m-%d").to_string());
    NaiveDate::parse_from_str(&value, "%Y-%m-%d")
        .map_err(|_| ApiError::BadRequest("date must use YYYY-MM-DD".to_owned()))?;
    Ok(value)
}

async fn record_event(
    state: &AppState,
    event_type: &str,
    lesson_slug: Option<&str>,
    exercise_slug: Option<&str>,
) -> ApiResult<()> {
    sqlx::query(
        "INSERT INTO learning_events (event_type, lesson_slug, exercise_slug)
         VALUES (?, ?, ?)",
    )
    .bind(event_type)
    .bind(lesson_slug)
    .bind(exercise_slug)
    .execute(&state.pool)
    .await?;
    Ok(())
}

async fn learning_streak(state: &AppState) -> ApiResult<usize> {
    let dates: Vec<String> = sqlx::query_scalar(
        "SELECT DISTINCT event_date FROM learning_events ORDER BY event_date DESC",
    )
    .fetch_all(&state.pool)
    .await?;
    let set: HashSet<String> = dates.into_iter().collect();
    let today = Utc::now().date_naive();
    let start = if set.contains(&today.format("%Y-%m-%d").to_string()) {
        today
    } else {
        today - Duration::days(1)
    };
    let mut streak = 0;
    loop {
        let date = (start - Duration::days(streak as i64))
            .format("%Y-%m-%d")
            .to_string();
        if !set.contains(&date) {
            break;
        }
        streak += 1;
    }
    Ok(streak)
}

fn rating_name(rating: ReviewRating) -> &'static str {
    match rating {
        ReviewRating::Forgotten => "forgotten",
        ReviewRating::Fuzzy => "fuzzy",
        ReviewRating::Remembered => "remembered",
    }
}

fn sql_datetime_from_day(day: i64) -> ApiResult<String> {
    chrono::DateTime::from_timestamp(day * 86_400, 0)
        .map(|value| value.format("%Y-%m-%d %H:%M:%S").to_string())
        .ok_or_else(|| ApiError::BadRequest("review date is out of range".to_owned()))
}

fn now_sql() -> String {
    Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub async fn migrate_legacy_progress(state: &AppState) -> ApiResult<u64> {
    #[derive(sqlx::FromRow)]
    struct LegacyRow {
        status: String,
        pattern_note: String,
        stuck_note: String,
        review_note: String,
    }
    let mut imported = 0;
    for lesson in &state.catalog.lessons {
        let legacy = sqlx::query_as::<_, LegacyRow>(
            "SELECT status, pattern_note, stuck_note, review_note
             FROM training_records WHERE article_slug = ?",
        )
        .bind(&lesson.article_slug)
        .fetch_optional(&state.pool)
        .await?;
        let Some(legacy) = legacy else {
            continue;
        };
        let status = if legacy.status == "reviewed" {
            "completed"
        } else if legacy.status == "learned" || legacy.status == "practiced" {
            "in_progress"
        } else {
            "not_started"
        };
        let note = [legacy.pattern_note, legacy.stuck_note, legacy.review_note]
            .into_iter()
            .filter(|part| !part.trim().is_empty())
            .collect::<Vec<_>>()
            .join("\n\n");
        let result = sqlx::query(
            "INSERT OR IGNORE INTO lesson_progress
             (lesson_slug, status, animation_completed, quiz_best_score, note, completed_at)
             VALUES (?, ?, ?, ?, ?, CASE WHEN ? = 'completed' THEN datetime('now') END)",
        )
        .bind(&lesson.slug)
        .bind(status)
        .bind(i64::from(status == "completed" && lesson.visualization_id.is_some()))
        .bind(if status == "completed" { 100 } else { 0 })
        .bind(note)
        .bind(status)
        .execute(&state.pool)
        .await?;
        if result.rows_affected() > 0 {
            imported += 1;
            if status == "completed" {
                let due_at = (Utc::now() + Duration::days(1))
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string();
                sqlx::query(
                    "INSERT OR IGNORE INTO review_schedules (lesson_slug, step, due_at)
                     VALUES (?, 0, ?)",
                )
                .bind(&lesson.slug)
                .bind(due_at)
                .execute(&state.pool)
                .await?;
            }
        }
    }
    Ok(imported)
}
