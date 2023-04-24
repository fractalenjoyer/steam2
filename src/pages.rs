use rocket_dyn_templates::{context, Template};

#[get("/download")]
pub fn download() -> Template {
    Template::render(
        "download",
        context! {
            title: "Download",
            style: "download.css",
        },
    )
}

#[get("/about")]
pub fn about() -> Template {
    Template::render(
        "about",
        context! {
            title: "About",
            style: "about.css",
        },
    )
}

#[get("/contact")]
pub fn contact() -> Template {
    Template::render(
        "contact",
        context! {
            title: "Contact",
            style: "contact.css",
        },
    )
}
