use crate::inserts::query_insert_msg;
use crate::DbPool;
use actix_web::{get, post, web, HttpResponse, Responder};
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

use crate::models::Carousel;
use crate::models::LayoutPost;
use crate::models::LayoutProfile;
use crate::models::LayoutService;
use crate::models::Msg;
use crate::models::NewPostHandler;
use crate::models::PContact;
use crate::models::Pindex;
use crate::models::Post;
use crate::models::Pprofile;
use crate::models::Profile;
use crate::models::Pservice;
use crate::models::Service;
use crate::models::Skilss;
use crate::models::Social;
use crate::models::Symbol;

use crate::schemas::carousel::dsl::*;
use crate::schemas::layoutpost::dsl::*;
use crate::schemas::layoutprofile::dsl::*;
use crate::schemas::layoutservice::dsl::*;
use crate::schemas::p_contact::dsl::*;
use crate::schemas::pindex::dsl::*;
use crate::schemas::posts::dsl::*;
use crate::schemas::pprofile::dsl::*;
use crate::schemas::profiles::dsl::*;
use crate::schemas::pservice::dsl::*;
use crate::schemas::services::dsl::*;
use crate::schemas::skills::dsl::*;
use crate::schemas::socials::dsl::*;
use crate::schemas::symbol::dsl::*;

//********************************************* ACTIX WEB IN USE FOR TEMPLATES*********************************************
#[get("/")]
pub async fn index(
    pool: web::Data<DbPool>,
    template_manager: web::Data<tera::Tera>,
) -> impl Responder {
    let mut conn = pool.get().expect("Problemas al traer la base de datos");
    let diesel_capture_dates_1 = carousel
        .load::<Carousel>(&mut conn)
        .expect("Error al ejecutar la query");
    let diesel_capture_dates_2 = symbol
        .load::<Symbol>(&mut conn)
        .expect("Error al ejecutar la query");
    let diesel_capture_dates_3 = pindex
        .load::<Pindex>(&mut conn)
        .expect("Error al ejecutar la query");

    let mut ctx = tera::Context::new();
    ctx.insert("carousel", &diesel_capture_dates_1);
    ctx.insert("symbol", &diesel_capture_dates_2);
    ctx.insert("pindex", &diesel_capture_dates_3);

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template_manager.render("index.html", &ctx).unwrap())
}

#[get("/service")]
pub async fn service(
    pool: web::Data<DbPool>,
    template_manager: web::Data<tera::Tera>,
) -> impl Responder {
    let mut conn = pool.get().expect("Problemas al traer la base de datos");
    let diesel_capture_dates_1 = layoutservice
        .load::<LayoutService>(&mut conn)
        .expect("Error al ejecutar la query");
    let diesel_capture_dates_2 = pservice
        .load::<Pservice>(&mut conn)
        .expect("Error al ejecutar la query");
    let diesel_capture_dates_3 = services
        .load::<Service>(&mut conn)
        .expect("Error al ejecutar la query");

    let mut ctx = tera::Context::new();
    ctx.insert("layoutservice", &diesel_capture_dates_1);
    ctx.insert("pservice", &diesel_capture_dates_2);
    ctx.insert("services", &diesel_capture_dates_3);

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template_manager.render("service.html", &ctx).unwrap())
}

#[get("/profile")]
pub async fn profile(
    pool: web::Data<DbPool>,
    template_manager: web::Data<tera::Tera>,
) -> impl Responder {
    let mut conn = pool.get().expect("Problemas al traer la base de datos");
    let diesel_capture_dates_1 = layoutprofile
        .load::<LayoutProfile>(&mut conn)
        .expect("Error al ejecutar la query");
    let diesel_capture_dates_2 = pprofile
        .load::<Pprofile>(&mut conn)
        .expect("Error al ejecutar la query");
    let diesel_capture_dates_3 = profiles
        .load::<Profile>(&mut conn)
        .expect("Error al ejecutar la query");
    let diesel_capture_dates_4 = skills
        .load::<Skilss>(&mut conn)
        .expect("Error al ejecutar la query");

    let mut ctx = tera::Context::new();
    ctx.insert("layoutprofile", &diesel_capture_dates_1);
    ctx.insert("pprofile", &diesel_capture_dates_2);
    ctx.insert("profiles", &diesel_capture_dates_3);
    ctx.insert("skills", &diesel_capture_dates_4);

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template_manager.render("profile.html", &ctx).unwrap())
}

#[get("/post")]
pub async fn post(
    pool: web::Data<DbPool>,
    template_manager: web::Data<tera::Tera>,
) -> impl Responder {
    let mut conn = pool.get().expect("Problemas al traer la base de datos");
    let diesel_capture_dates_1 = posts
        .load::<Post>(&mut conn)
        .expect("Error al ejecutar la query");
    let diesel_capture_dates_2 = layoutpost
        .load::<LayoutPost>(&mut conn)
        .expect("Error al ejecutar la query");

    let mut ctx = tera::Context::new();
    ctx.insert("posts", &diesel_capture_dates_1);
    ctx.insert("layoutpost", &diesel_capture_dates_2);

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template_manager.render("post.html", &ctx).unwrap())
}

#[get("/contact")]
pub async fn contact(
    pool: web::Data<DbPool>,
    template_manager: web::Data<tera::Tera>,
) -> impl Responder {
    let mut conn = pool.get().expect("Problemas al traer la base de datos");
    let diesel_capture_dates_1 = p_contact
        .load::<PContact>(&mut conn)
        .expect("Error al ejecutar la query");
    let diesel_capture_dates_2 = socials
        .load::<Social>(&mut conn)
        .expect("Error al ejecutar la query");

    let mut ctx = tera::Context::new();
    ctx.insert("p_contact", &diesel_capture_dates_1);
    ctx.insert("socials", &diesel_capture_dates_2);
    ctx.insert("Msg", "Submit");

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template_manager.render("contact.html", &ctx).unwrap())
}

#[post("/contact")]
pub async fn contact_post(data: web::Form<Msg>) -> impl Responder {
    println!(
        "Contact Post  
        firstName:{},
        lastName:{},
        email:{},
        country: {},
        message:{} 
    ",
        data.first_name, data.last_name, data.email, data.country, data.message
    );

    query_insert_msg(
        &data.first_name,
        &data.last_name,
        &data.email,
        &data.country,
        &data.message,
    );

    HttpResponse::Ok().body(format!(
        "Contact Post Tank You Contact Test:
        firstName:{},
        lastName:{},
        email:{},
        country: {},
        message:{} 
    ",
        data.first_name, data.last_name, data.email, data.country, data.message
    ))
}

#[get("/blog/{query_index_get}")]
pub async fn query_index_get(
    pool: web::Data<DbPool>,
    template_manager: web::Data<tera::Tera>,
    blog_slug: web::Path<String>,
) -> impl Responder {
    let mut conn = pool.get().expect("Problemas al traer la base de datos");

    let url_slug = blog_slug.into_inner();

    match web::block(move || posts.filter(slug.eq(url_slug)).load::<Post>(&mut conn)).await {
        Ok(data) => {
            let data = data.unwrap();

            if data.len() == 0 {
                return HttpResponse::NotFound().finish();
            }

            let data = &data[0];

            let mut ctx = tera::Context::new();
            ctx.insert("post", data);

            HttpResponse::Ok()
                .content_type("text/html")
                .body(template_manager.render("posts.html", &ctx).unwrap())

            // return HttpResponse::Ok().body(format!("{:?}", data));
        }
        Err(_err) => HttpResponse::Ok().body("Error al recibir la data"),
    }
}

#[post("/query_index_post")]
pub async fn query_index_post(
    pool: web::Data<DbPool>,
    item: web::Json<NewPostHandler>,
) -> impl Responder {
    println!("{:?}", item);
    let mut conn = pool.get().expect("Problemas al traer la base de datos");

    match web::block(move || Post::create_post(&mut conn, &item)).await {
        Ok(data) => {
            return HttpResponse::Ok().body(format!("{:?}", data));
        }
        Err(_err) => HttpResponse::Ok().body("Error al recibir la data"),
    }
}
//********************************************* END ACTIX WEB IN USE FOR TEMPLATES*********************************************
