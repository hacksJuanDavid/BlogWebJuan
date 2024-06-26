use crate::schemas::posts;
use diesel::prelude::*;
use serde::{Deserialize, Serialize}; //Permite convertir un objeto a json y viceversa

// Structs Index Start ----------------------------------------------

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Pindex {
    pub id_p_index: i32,
    pub title_p_index: String,
    pub sub_title_p_index: String,
    pub p_p_index_1: String,
    pub p_p_index_2: String,
    pub p_p_index_3: String,
    pub img_p_index: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schemas::pindex)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPindex<'a> {
    pub title_p_index: &'a str,
    pub sub_title_p_index: &'a str,
    pub p_p_index_1: &'a str,
    pub p_p_index_2: &'a str,
    pub p_p_index_3: &'a str,
    pub img_p_index: &'a str,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Symbol {
    pub id_symbol: i32,
    pub title_symbol: String,
    pub p_symbol: String,
    pub img_symbol: String,
    pub href_symbol: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schemas::symbol)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewSymbol<'a> {
    pub title_symbol: &'a str,
    pub p_symbol: &'a str,
    pub img_symbol: &'a str,
    pub href_symbol: &'a str,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Carousel {
    pub id_carousel: i32,
    pub title_carousel_1: String,
    pub title_carousel_2: String,
    pub title_carousel_3: String,
    pub p_carousel_1: String,
    pub p_carousel_2: String,
    pub p_carousel_3: String,
    pub img_carousel_1: String,
    pub img_carousel_2: String,
    pub img_carousel_3: String,
    pub href_carousel_1: String,
    pub href_carousel_2: String,
    pub href_carousel_3: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schemas::carousel)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewCarousel<'a> {
    pub title_carousel_1: &'a str,
    pub title_carousel_2: &'a str,
    pub title_carousel_3: &'a str,
    pub p_carousel_1: &'a str,
    pub p_carousel_2: &'a str,
    pub p_carousel_3: &'a str,
    pub img_carousel_1: &'a str,
    pub img_carousel_2: &'a str,
    pub img_carousel_3: &'a str,
    pub href_carousel_1: &'a str,
    pub href_carousel_2: &'a str,
    pub href_carousel_3: &'a str,
}

// Structs Index Final ----------------------------------------------

// Structs Service Start ----------------------------------------------
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct LayoutService {
    pub id_layout_service: i32,
    pub title_layout_service: String,
    pub p_p_layout_service: String,
    pub img_layout_service: String,
    pub title_button_layout_service_1: String,
    pub title_button_layout_service_2: String,
    pub href_layout_service_1: String,
    pub href_layout_service_2: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schemas::layoutservice)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewLayoutService<'a> {
    pub title_layout_service: &'a str,
    pub p_p_layout_service: &'a str,
    pub img_layout_service: &'a str,
    pub title_button_layout_service_1: &'a str,
    pub title_button_layout_service_2: &'a str,
    pub href_layout_service_1: &'a str,
    pub href_layout_service_2: &'a str,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Pservice {
    pub id_p_service: i32,
    pub title_p_service: String,
    pub sub_title_p_service: String,
    pub p_p_service_1: String,
    pub p_p_service_2: String,
    pub p_p_service_3: String,
    pub img_p_service: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schemas::pservice)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewParrafoService<'a> {
    pub title_p_service: &'a str,
    pub sub_title_p_service: &'a str,
    pub p_p_service_1: &'a str,
    pub p_p_service_2: &'a str,
    pub p_p_service_3: &'a str,
    pub img_p_service: &'a str,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Service {
    pub id_service: i32,
    pub name: String,
    pub description: String,
    pub img_service_1: String,
    pub img_service_2: String,
    pub img_service_3: String,
    pub published_service: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schemas::services)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewService<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub img_service_1: &'a str,
    pub img_service_2: &'a str,
    pub img_service_3: &'a str,
    pub published_service: bool,
}
// Structs Service Final ----------------------------------------------

// Structs Profile Start ----------------------------------------------
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct LayoutProfile {
    pub id_layout_profile: i32,
    pub title_layout_profile: String,
    pub p_p_layout_profile: String,
    pub img_layout_profile: String,
    pub title_button_layout_profile_1: String,
    pub title_button_layout_profile_2: String,
    pub href_layout_profile_1: String,
    pub href_layout_profile_2: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schemas::layoutprofile)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewLayoutprofile<'a> {
    pub title_layout_profile: &'a str,
    pub p_p_layout_profile: &'a str,
    pub img_layout_profile: &'a str,
    pub title_button_layout_profile_1: &'a str,
    pub title_button_layout_profile_2: &'a str,
    pub href_layout_profile_1: &'a str,
    pub href_layout_profile_2: &'a str,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Pprofile {
    pub id_p_profile: i32,
    pub title_p_profile: String,
    pub sub_title_p_profile: String,
    pub p_p_profile_1: String,
    pub p_p_profile_2: String,
    pub p_p_profile_3: String,
    pub img_p_profile: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schemas::pprofile)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewParrafoProfile<'a> {
    pub title_p_profile: &'a str,
    pub sub_title_p_profile: &'a str,
    pub p_p_profile_1: &'a str,
    pub p_p_profile_2: &'a str,
    pub p_p_profile_3: &'a str,
    pub img_p_profile: &'a str,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Profile {
    pub id_profile: i32,
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub git_hub: String,
    pub phone: String,
    pub address: String,
    pub city: String,
    pub country: String,
    pub text: String,
    pub published_profile: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schemas::profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewProfile<'a> {
    pub name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub git_hub: &'a str,
    pub phone: &'a str,
    pub address: &'a str,
    pub city: &'a str,
    pub country: &'a str,
    pub text: &'a str,
    pub published_profile: &'a bool,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Skilss {
    pub id_skill: i32,
    pub title_skill: String,
    pub name_skill: String,
    pub level_skill: String,
    pub type_skill: String,
    pub img_skill: String,
    pub img_perfil_skill: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schemas::skills)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewSkill<'a> {
    pub title_skill: &'a str,
    pub name_skill: &'a str,
    pub level_skill: &'a str,
    pub type_skill: &'a str,
    pub img_skill: &'a str,
    pub img_perfil_skill: &'a str,
}

// Structs Profile Final ----------------------------------------------

// Structs Post Start ----------------------------------------------
//Structs Post layout
#[derive(Queryable, Serialize, Deserialize)]
pub struct LayoutPost {
    pub id_layout_post: i32,
    pub title_layout_post: String,
    pub p_p_layout_post: String,
    pub img_layout_post_1: String,
    pub img_layout_post_2: String,
    pub title_button_layout_post_1: String,
    pub href_layout_post_1: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schemas::layoutpost)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewLayoutpost<'a> {
    pub title_layout_post: &'a str,
    pub p_p_layout_post: &'a str,
    pub img_layout_post_1: &'a str,
    pub img_layout_post_2: &'a str,
    pub title_button_layout_post_1: &'a str,
    pub href_layout_post_1: &'a str,
}

//Generador de las Query, esto puede ser una row de el sql o una tabla de postgresql
#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
    pub img: String,
    pub date: String,
    pub published: bool,
}

//usar el schema creado traerlo desde schaema.rs

//Macro para crear un nuevo post
#[derive(Insertable)]
//Macro para asignarle un nombre a la tabla
#[diesel(table_name = crate::schemas::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
//Generar de query insertable
//El  id es auto generado por postgresql no se necesita asignarle un valor.
pub struct NewPost<'a> {
    pub title: &'a str,
    pub slug: &'a str,
    pub body: &'a str,
    pub img: &'a str,
    pub date: &'a str,
    pub published: &'a bool,
}

//Convertir archivos Json a Rust struct
//Modelo de serializacion para poder agregar post a la base de datos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPostHandler {
    pub title: String,
    pub slug: String,
    pub body: String,
    pub img: String,
    pub date: String,
    pub published: bool,
}

//implementaremos una funcion para los post
impl Post {
    pub fn slugify(title: &String) -> String {
        return title.replace(" ", "-").to_lowercase();
    }

    pub fn create_post<'a>(
        conn: &mut PgConnection,
        post: &NewPostHandler,
    ) -> Result<Post, diesel::result::Error> {
        let _slug = Post::slugify(&post.title.clone());

        let new_post = NewPost {
            title: &post.title,
            slug: &post.slug,
            body: &post.body,
            img: &post.img,
            date: &post.date,
            published: &post.published,
        };

        diesel::insert_into(posts::table)
            .values(new_post)
            .get_result::<Post>(conn)
    }
}
// Structs Post Final ----------------------------------------------

// Structs Contact Start ----------------------------------------------
#[derive(Queryable, Serialize, Deserialize)]
pub struct PContact {
    pub id_p_contact: i32,
    pub title_p_contact: String,
    pub p_p_contact_1: String,
    pub p_p_contact_2: String,
    pub p_p_contact_3: String,
    pub title_button_contact_1: String,
    pub title_button_contact_2: String,
    pub title_button_contact_3: String,
    pub href_contact_1: String,
    pub href_contact_2: String,
    pub href_contact_3: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schemas::p_contact)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPcontact<'a> {
    pub title_p_contact: &'a str,
    pub p_p_contact_1: &'a str,
    pub p_p_contact_2: &'a str,
    pub p_p_contact_3: &'a str,
    pub title_button_contact_1: &'a str,
    pub title_button_contact_2: &'a str,
    pub title_button_contact_3: &'a str,
    pub href_contact_1: &'a str,
    pub href_contact_2: &'a str,
    pub href_contact_3: &'a str,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Msg {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub country: String,
    pub message: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schemas::msg)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMsg<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub country: &'a str,
    pub message: &'a str,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Social {
    pub id_socials: i32,
    pub name: String,
    pub telegram: String,
    pub linkedin: String,
    pub github: String,
    pub instagram: String,
    pub whatsapp: String,
    pub twitter: String,
    pub published_social: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schemas::socials)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewSocial<'a> {
    pub name: &'a str,
    pub telegram: &'a str,
    pub linkedin: &'a str,
    pub github: &'a str,
    pub instagram: &'a str,
    pub whatsapp: &'a str,
    pub twitter: &'a str,
    pub published_social: bool,
}
// Structs Contact Final ----------------------------------------------
