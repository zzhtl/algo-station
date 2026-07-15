//! 课程完成与间隔复习的纯领域规则。

use serde::{Deserialize, Serialize};

/// 判断一节课是否达到完成条件所需的证据。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompletionEvidence {
    pub best_quiz_score: u8,
    pub core_exercise_accepted: bool,
    pub visualization_required: bool,
    pub visualization_completed: bool,
}

/// 小测至少 80 分、通过核心题，并在需要时走完动态图。
pub fn lesson_is_complete(evidence: &CompletionEvidence) -> bool {
    evidence.best_quiz_score >= 80
        && evidence.core_exercise_accepted
        && (!evidence.visualization_required || evidence.visualization_completed)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewRating {
    Forgotten,
    Fuzzy,
    Remembered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewSchedule {
    pub level: u8,
    pub due_day: i64,
    pub mastered: bool,
}

impl ReviewSchedule {
    const INTERVALS: [i64; 5] = [1, 3, 7, 14, 30];

    pub fn initial(completed_day: i64) -> Self {
        Self {
            level: 0,
            due_day: completed_day + Self::INTERVALS[0],
            mastered: false,
        }
    }

    pub fn after(self, rating: ReviewRating, reviewed_day: i64) -> Self {
        match rating {
            ReviewRating::Forgotten => Self::initial(reviewed_day),
            ReviewRating::Fuzzy => {
                let interval = Self::INTERVALS[usize::from(self.level)];
                Self {
                    due_day: reviewed_day + (interval + 1) / 2,
                    ..self
                }
            }
            ReviewRating::Remembered => {
                let level = self.level.saturating_add(1).min(4);
                Self {
                    level,
                    due_day: reviewed_day + Self::INTERVALS[usize::from(level)],
                    mastered: level == 4,
                }
            }
        }
    }
}
