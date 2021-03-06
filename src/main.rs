#![feature(plugin, custom_derive, decl_macro)]
#![plugin(rocket_codegen)]
extern crate ip_manager;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;

use rocket::request::LenientForm;
use ip_manager::{handle_command, handle_submission};
use ip_manager::slack::slash_command::Request;
use ip_manager::slack::dialog::{Submission, SubmissionResponse};

fn main() {
    try_main().unwrap();
}

fn try_main() -> Result<(), Box<std::error::Error>> {
    rocket::ignite()
        .mount("/ip-manager/command", routes![command_request])
        .mount("/ip-manager/submission", routes![dialog_response])
        .launch();
    Ok(())
}

#[post("/<command>", data = "<form>")]
fn command_request(
    command: String,
    form: LenientForm<Request>,
) -> Result<rocket_contrib::Json, Box<std::error::Error>> {
    let data = form.into_inner();
    let json = handle_command(&command, data)?;
    Ok(rocket_contrib::Json(json))
}

#[post("/", data = "<form>")]
fn dialog_response(
    form: LenientForm<SubmissionResponse>,
) -> Result<String, Box<std::error::Error>> {
    let data: Submission = serde_json::from_str(&form.into_inner().payload).unwrap();
    handle_submission(data)?;
    Ok("".to_owned())
}
