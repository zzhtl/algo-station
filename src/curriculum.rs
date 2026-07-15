//! 编译进二进制的课程、练习与动态图目录。

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

const BUNDLED_CATALOG: &str = include_str!("../content/curriculum/catalog.json");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurriculumCatalog {
    pub schema_version: u32,
    pub stages: Vec<StageDefinition>,
    pub lessons: Vec<LessonDefinition>,
    pub exercises: Vec<ExerciseDefinition>,
    pub visualizations: Vec<VisualizationDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageDefinition {
    pub id: String,
    pub title: String,
    pub description: String,
    pub order: u16,
    pub lesson_slugs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonDefinition {
    pub slug: String,
    pub stage_id: String,
    pub article_slug: String,
    pub title: String,
    pub summary: String,
    pub order: u16,
    pub estimated_minutes: u16,
    #[serde(default)]
    pub prerequisites: Vec<String>,
    pub visualization_id: Option<String>,
    pub objectives: Vec<String>,
    pub quiz: Vec<QuizQuestion>,
    pub exercise_slugs: Vec<String>,
    pub core_exercise_slugs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizQuestion {
    pub id: String,
    pub prompt: String,
    pub options: Vec<String>,
    pub correct_index: usize,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseDefinition {
    pub slug: String,
    pub problem_id: i64,
    pub lesson_slug: String,
    pub title: String,
    pub difficulty: String,
    pub summary: String,
    pub starters: Vec<StarterTemplate>,
    pub cases: Vec<ExerciseCase>,
    pub limits: ExerciseLimits,
}

impl ExerciseDefinition {
    pub fn has_template(&self, language: &str, contract: &str) -> bool {
        self.starters
            .iter()
            .any(|item| item.language == language && item.contract == contract)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarterTemplate {
    pub language: String,
    pub contract: String,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseCase {
    pub name: String,
    pub visibility: String,
    pub input: String,
    pub expected: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseLimits {
    pub compile_ms: u64,
    pub case_ms: u64,
    pub total_ms: u64,
    pub memory_mb: u64,
    pub output_kb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationDefinition {
    pub id: String,
    pub lesson_slug: String,
    pub title: String,
    pub kind: String,
    pub description: String,
}

#[derive(Debug, thiserror::Error)]
pub enum CatalogError {
    #[error("课程目录 JSON 无法解析: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("课程目录校验失败: {0}")]
    Invalid(String),
}

impl CurriculumCatalog {
    pub fn bundled() -> Result<Self, CatalogError> {
        serde_json::from_str(BUNDLED_CATALOG).map_err(Into::into)
    }

    pub fn lesson_count(&self) -> usize {
        self.lessons.len()
    }

    pub fn exercise_count(&self) -> usize {
        self.exercises.len()
    }

    pub fn visualization_count(&self) -> usize {
        self.visualizations.len()
    }

    pub fn validate(&self) -> Result<(), CatalogError> {
        ensure_unique(self.stages.iter().map(|item| item.id.as_str()), "阶段")?;
        ensure_unique(self.lessons.iter().map(|item| item.slug.as_str()), "课程")?;
        ensure_unique(self.exercises.iter().map(|item| item.slug.as_str()), "练习")?;
        ensure_unique(
            self.visualizations.iter().map(|item| item.id.as_str()),
            "动态图",
        )?;

        let lessons: HashSet<&str> = self.lessons.iter().map(|item| item.slug.as_str()).collect();
        let exercises: HashSet<&str> = self
            .exercises
            .iter()
            .map(|item| item.slug.as_str())
            .collect();
        let visualizations: HashSet<&str> = self
            .visualizations
            .iter()
            .map(|item| item.id.as_str())
            .collect();

        for stage in &self.stages {
            for slug in &stage.lesson_slugs {
                if !lessons.contains(slug.as_str()) {
                    return Err(CatalogError::Invalid(format!(
                        "阶段 {} 引用了不存在的课程 {}",
                        stage.id, slug
                    )));
                }
            }
        }
        for lesson in &self.lessons {
            if let Some(id) = &lesson.visualization_id
                && !visualizations.contains(id.as_str())
            {
                return Err(CatalogError::Invalid(format!(
                    "课程 {} 引用了不存在的动态图 {}",
                    lesson.slug, id
                )));
            }
            if lesson.quiz.len() < 3 {
                return Err(CatalogError::Invalid(format!(
                    "课程 {} 的小测少于 3 题",
                    lesson.slug
                )));
            }
            if lesson.core_exercise_slugs.is_empty() {
                return Err(CatalogError::Invalid(format!(
                    "课程 {} 没有核心练习",
                    lesson.slug
                )));
            }
            for slug in lesson
                .exercise_slugs
                .iter()
                .chain(lesson.core_exercise_slugs.iter())
            {
                if !exercises.contains(slug.as_str()) {
                    return Err(CatalogError::Invalid(format!(
                        "课程 {} 引用了不存在的练习 {}",
                        lesson.slug, slug
                    )));
                }
            }
        }
        for exercise in &self.exercises {
            if !lessons.contains(exercise.lesson_slug.as_str()) {
                return Err(CatalogError::Invalid(format!(
                    "练习 {} 引用了不存在的课程 {}",
                    exercise.slug, exercise.lesson_slug
                )));
            }
        }

        self.validate_prerequisites()
    }

    pub fn validate_prerequisites(&self) -> Result<(), CatalogError> {
        let positions: HashMap<&str, usize> = self
            .lessons
            .iter()
            .enumerate()
            .map(|(index, item)| (item.slug.as_str(), index))
            .collect();

        for (index, lesson) in self.lessons.iter().enumerate() {
            for prerequisite in &lesson.prerequisites {
                let Some(prerequisite_index) = positions.get(prerequisite.as_str()) else {
                    return Err(CatalogError::Invalid(format!(
                        "课程 {} 的前置 {} 不存在",
                        lesson.slug, prerequisite
                    )));
                };
                if *prerequisite_index >= index {
                    return Err(CatalogError::Invalid(format!(
                        "课程 {} 的前置 {} 没有位于它之前",
                        lesson.slug, prerequisite
                    )));
                }
            }
        }
        Ok(())
    }
}

fn ensure_unique<'a>(values: impl Iterator<Item = &'a str>, label: &str) -> Result<(), CatalogError> {
    let mut seen = HashSet::new();
    for value in values {
        if !seen.insert(value) {
            return Err(CatalogError::Invalid(format!("{label} slug 重复: {value}")));
        }
    }
    Ok(())
}
