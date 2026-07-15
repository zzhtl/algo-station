use algo_station::curriculum::CurriculumCatalog;

#[test]
fn bundled_curriculum_has_the_committed_scope() {
    let catalog = CurriculumCatalog::bundled().expect("bundled curriculum must parse");

    assert_eq!(catalog.stages.len(), 12);
    assert_eq!(catalog.lesson_count(), 100);
    assert_eq!(catalog.exercise_count(), 150);
    assert_eq!(catalog.visualization_count(), 25);
    catalog
        .validate()
        .expect("bundled curriculum must satisfy all references and invariants");
}

#[test]
fn every_exercise_supports_both_languages_and_contracts() {
    let catalog = CurriculumCatalog::bundled().expect("bundled curriculum must parse");

    for exercise in &catalog.exercises {
        for language in ["go", "rust"] {
            for contract in ["function", "stdio"] {
                assert!(
                    exercise.has_template(language, contract),
                    "{} is missing {language}/{contract}",
                    exercise.slug
                );
            }
        }
        assert!(!exercise.cases.is_empty(), "{} has no cases", exercise.slug);
        assert!(
            exercise.cases.iter().any(|case| case.visibility == "hidden"),
            "{} has no hidden case",
            exercise.slug
        );
        let public = exercise
            .cases
            .iter()
            .find(|case| case.visibility == "public")
            .expect("exercise must expose a public example");
        for hidden in exercise
            .cases
            .iter()
            .filter(|case| case.visibility == "hidden")
        {
            assert_ne!(
                (&hidden.input, &hidden.expected),
                (&public.input, &public.expected),
                "{} reuses the public example as a hidden case",
                exercise.slug
            );
        }
    }
}

#[test]
fn prerequisites_are_acyclic_and_point_to_earlier_lessons() {
    let catalog = CurriculumCatalog::bundled().expect("bundled curriculum must parse");
    catalog
        .validate_prerequisites()
        .expect("prerequisites must form an acyclic ordered graph");
}
