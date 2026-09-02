use crate::models::*;
use axum::{response::IntoResponse, Json};

// ---- CRUD for Questions ----

pub async fn create_question(Json(question): Json<Question>) -> impl IntoResponse {
    Json(QuestionDetail {
        question_uuid: "d347261c-3f0e-42d2-8706-5ef9f1b96725".to_owned(),
        title: "Newly Created Question".to_owned(),
        description: "My Description".to_owned(),
        created_at: "2022-12-31 18:44:08.287442".to_owned(),
    })
}

pub async fn read_questions() -> impl IntoResponse {
    let () = todo!();
}

pub async fn delete_question(Json(question_uuid): Json<QuestionId>) {
    let () = todo!();
}

// ---- CRUD for Answers ----

// TODO: Create a POST route to /answer which accepts an `Answer` and returns `AnswerDetail` as JSON.
//       The handler function should be called `create_answer`.
//
//       hint: this function should look very similar to the create_question function above
pub async fn create_answer(Json(answer): Json<Answer>) -> impl IntoResponse {
    let () = todo!();
}

// TODO: Create a GET route to /answers which accepts an `QuestionId` and returns a vector of `AnswerDetail` as JSON.
//       The handler function should be called `read_answers`.
//
//       hint: this function should look very similar to the read_questions function above
pub async fn read_answers() -> impl IntoResponse {
    let () = todo!();
}

// TODO: Create a DELETE route to /answer which accepts an `AnswerId` and does not return anything.
//       The handler function should be called `delete_answer`.
//
//       hint: this function should look very similar to the delete_question function above
pub async fn delete_answer(Json(answer_uuid): Json<AnswerId>) {
    let () = todo!();
}
