table! {
    defect (id) {
        id -> Nullable<Text>,
        project_id -> Nullable<Text>,
        description -> Binary,
    }
}

table! {
    defect_feature (defect_id, feature_id) {
        defect_id -> Nullable<Text>,
        feature_id -> Nullable<Text>,
    }
}

table! {
    defect_user_story (defect_id, user_story_id) {
        defect_id -> Nullable<Text>,
        user_story_id -> Nullable<Text>,
    }
}

table! {
    feature (id) {
        id -> Nullable<Text>,
        release_id -> Text,
        name -> Text,
    }
}

table! {
    project (id) {
        id -> Text,
        name -> Text,
    }
}

table! {
    release (id) {
        id -> Nullable<Text>,
        project_id -> Text,
        version -> Text,
    }
}

table! {
    user_story (id) {
        id -> Nullable<Text>,
        feature_id -> Text,
        name -> Text,
    }
}

joinable!(defect -> project (project_id));
joinable!(defect_feature -> defect (defect_id));
joinable!(defect_feature -> feature (feature_id));
joinable!(defect_user_story -> defect (defect_id));
joinable!(defect_user_story -> user_story (user_story_id));
joinable!(feature -> release (release_id));
joinable!(release -> project (project_id));
joinable!(user_story -> feature (feature_id));

allow_tables_to_appear_in_same_query!(
    defect,
    defect_feature,
    defect_user_story,
    feature,
    project,
    release,
    user_story,
);
