use rocket_dyn_templates::{context, Template};

#[get("/download")]
pub fn download() -> Option<Template> {
    Some(Template::render(
        "download",
        context! {
            title: "Download",
            style: "download.css",
        },
    ))
}

#[get("/about")]
pub fn about() -> Option<Template> {
    Some(Template::render(
        "about",
        context! {
            title: "About",
            style: "about.css",
        },
    ))
}

#[get("/contact")]
pub fn contact() -> Option<Template> {
    Some(Template::render(
        "contact",
        context! {
            title: "Contact",
            style: "contact.css",
        },
    ))
}