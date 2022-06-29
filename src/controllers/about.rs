use rocket_contrib::templates::Template;

#[derive(serde::Serialize)]
struct AboutContext {
   parent: &'static str
}
pub fn index() -> Template{
    let context = AboutContext{parent: super::DEFAULT_LAYOUT};
    Template::render("about/index", &context)
}
