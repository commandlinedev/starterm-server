#![deny(warnings)]
use std::sync::Arc;

use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;
use starterm::Filter;

struct WithTemplate<T: Serialize> {
    name: &'static str,
    value: T,
}

fn render<T>(template: WithTemplate<T>, hbs: Arc<Handlebars<'_>>) -> impl starterm::Reply
where
    T: Serialize,
{
    let render = hbs
        .render(template.name, &template.value)
        .unwrap_or_else(|err| err.to_string());
    starterm::reply::html(render)
}

#[tokio::main]
async fn main() {
    let template = "<!DOCTYPE html>
                    <html>
                      <head>
                        <title>Starterm Handlebars template example</title>
                      </head>
                      <body>
                        <h1>Hello {{user}}!</h1>
                      </body>
                    </html>";

    let mut hb = Handlebars::new();
    // register the template
    hb.register_template_string("template.html", template)
        .unwrap();

    // Turn Handlebars instance into a Filter so we can combine it
    // easily with others...
    let hb = Arc::new(hb);

    // Create a reusable closure to render template
    let handlebars = move |with_template| render(with_template, hb.clone());

    //GET /
    let route = starterm::get()
        .and(starterm::path::end())
        .map(|| WithTemplate {
            name: "template.html",
            value: json!({"user" : "Starterm"}),
        })
        .map(handlebars);

    starterm::serve(route).run(([127, 0, 0, 1], 3030)).await;
}
