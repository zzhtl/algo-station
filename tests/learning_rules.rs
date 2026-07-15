use algo_station::learning::{
    CompletionEvidence, ReviewRating, ReviewSchedule, lesson_is_complete,
};

#[test]
fn lesson_requires_quiz_core_acceptance_and_optional_visualization() {
    let incomplete = CompletionEvidence {
        best_quiz_score: 79,
        core_exercise_accepted: true,
        visualization_required: true,
        visualization_completed: true,
    };
    assert!(!lesson_is_complete(&incomplete));

    let complete = CompletionEvidence {
        best_quiz_score: 80,
        ..incomplete
    };
    assert!(lesson_is_complete(&complete));

    let animation_missing = CompletionEvidence {
        visualization_completed: false,
        ..complete
    };
    assert!(!lesson_is_complete(&animation_missing));

    let lesson_without_animation = CompletionEvidence {
        visualization_required: false,
        ..animation_missing
    };
    assert!(lesson_is_complete(&lesson_without_animation));
}

#[test]
fn spaced_review_uses_the_committed_intervals() {
    let initial = ReviewSchedule::initial(100);
    assert_eq!(initial.level, 0);
    assert_eq!(initial.due_day, 101);

    let remembered = initial.after(ReviewRating::Remembered, 101);
    assert_eq!(remembered.level, 1);
    assert_eq!(remembered.due_day, 104);

    let fuzzy = remembered.after(ReviewRating::Fuzzy, 104);
    assert_eq!(fuzzy.level, 1);
    assert_eq!(fuzzy.due_day, 106);

    let forgotten = fuzzy.after(ReviewRating::Forgotten, 106);
    assert_eq!(forgotten.level, 0);
    assert_eq!(forgotten.due_day, 107);
}

#[test]
fn fifth_success_marks_mastery_and_keeps_monthly_maintenance() {
    let mut schedule = ReviewSchedule::initial(0);
    let mut today = schedule.due_day;
    for _ in 0..5 {
        schedule = schedule.after(ReviewRating::Remembered, today);
        today = schedule.due_day;
    }

    assert!(schedule.mastered);
    assert_eq!(schedule.level, 4);

    let maintenance_day = schedule.due_day;
    let maintained = schedule.after(ReviewRating::Remembered, maintenance_day);
    assert!(maintained.mastered);
    assert_eq!(maintained.due_day, maintenance_day + 30);
}
