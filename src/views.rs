use crate::errors::IOracleResult;
use crate::models::{Answer, Hexagram, Trigram};
use crate::{iching, wires, Config, Db};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::templates::Template;

#[derive(FromForm)]
pub struct Data {
    email: String,
    question: String,
}

#[derive(Serialize)]
struct NoContext {}

#[derive(Serialize)]
pub struct FullAnswer {
    pub answer: Answer,
    pub hexagram: Hexagram,
    pub related: Hexagram,
    pub core_primary: Hexagram,
    pub core_related: Hexagram,
    pub first_trigram: Trigram,
    pub second_trigram: Trigram,
    pub first_related: Trigram,
    pub second_related: Trigram,
    pub core_primary_first: Trigram,
    pub core_primary_second: Trigram,
    pub core_related_first: Trigram,
    pub core_related_second: Trigram,
    pub generative_answer: String,
}

#[get("/")]
pub fn index() -> Template {
    Template::render("index", NoContext {})
}

#[post("/question", data = "<data>")]
pub fn question(
    connection: Db,
    config: State<Config>,
    data: Form<Data>,
) -> IOracleResult<Redirect> {
    let (hexagram, related) = wires::reading();
    let new_answer = Answer::new(&data.email, &data.question, &hexagram, &related);
    let answer_id = new_answer.id.clone();
    Answer::insert(&connection, &new_answer)?;
    Answer::send(
        &data.email,
        &data.question,
        &config.username,
        &config.password,
        &iching::full_answer(&connection, new_answer)?,
    );

    Ok(Redirect::to(format!("/answer/{}", answer_id)))
}

#[get("/answer/<id>")]
pub fn answer(connection: Db, id: String) -> IOracleResult<Template> {
    let answer = Answer::get_by_id(&connection, id)?;
    Ok(Template::render(
        "answer",
        iching::full_answer(&connection, answer)?,
    ))
}

#[catch(404)]
pub fn not_found() -> Redirect {
    Redirect::to("/")
}

#[catch(500)]
pub fn internal_error() -> Redirect {
    Redirect::to("/")
}
