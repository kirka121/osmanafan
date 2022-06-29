use rocket_contrib::templates::Template;

#[derive(serde::Serialize)]
struct IndexContext {
   parent: &'static str
}
pub fn index() -> Template{
    let context = IndexContext{parent: super::DEFAULT_LAYOUT};
    Template::render("home/index", &context)
}