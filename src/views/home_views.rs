use actix_web::{ get, web, HttpResponse };
use tera::{ Context, Tera };


#[get("/")]
pub async fn index_view(tera: web::Data<Tera>) -> HttpResponse {
    let mut context = Context::new();

    context.insert("from_view", "INDEX");

    let html = tera.render("app_home/index.html", &context).unwrap();
    HttpResponse::Ok().body(html)
}