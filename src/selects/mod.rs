use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

use crate::database::connect_db;
use crate::models::Carousel;
use crate::models::LayoutPost;
use crate::models::LayoutProfile;
use crate::models::LayoutService;
use crate::models::Msg;
use crate::models::NewCarousel;
use crate::models::NewLayoutService;
use crate::models::NewLayoutpost;
use crate::models::NewLayoutprofile;
use crate::models::NewMsg;
use crate::models::NewParrafoProfile;
use crate::models::NewParrafoService;
use crate::models::NewPcontact;
use crate::models::NewPindex;
use crate::models::NewPost;
use crate::models::NewProfile;
use crate::models::NewService;
use crate::models::NewSkill;
use crate::models::NewSocial;
use crate::models::NewSymbol;
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

use crate::schemas::carousel;
use crate::schemas::carousel::dsl::*;
use crate::schemas::layoutpost;
use crate::schemas::layoutpost::dsl::*;
use crate::schemas::layoutprofile;
use crate::schemas::layoutprofile::dsl::*;
use crate::schemas::layoutservice;
use crate::schemas::layoutservice::dsl::*;
use crate::schemas::msg;
use crate::schemas::msg::dsl::*;
use crate::schemas::p_contact;
use crate::schemas::p_contact::dsl::*;
use crate::schemas::pindex;
use crate::schemas::pindex::dsl::*;
use crate::schemas::posts;
use crate::schemas::posts::dsl::*;
use crate::schemas::pprofile;
use crate::schemas::pprofile::dsl::*;
use crate::schemas::profiles;
use crate::schemas::profiles::dsl::*;
use crate::schemas::pservice;
use crate::schemas::pservice::dsl::*;
use crate::schemas::services;
use crate::schemas::services::dsl::*;
use crate::schemas::skills;
use crate::schemas::skills::dsl::*;
use crate::schemas::socials;
use crate::schemas::socials::dsl::*;
use crate::schemas::symbol;
use crate::schemas::symbol::dsl::*;

//*********************************************Consultas Select Template Index*********************************************
//Start Funcion Seleccionar todos los Carousel
pub fn query_select_all_carousel() {
    println!("Query Select All Initial Carousel");
    let mut conn = connect_db(); //Conectamos a la base de datos
                                 //Select * from posts
    let carousel_result = carousel
        .order(id_carousel)
        .load::<Carousel>(&mut conn)
        .expect("Error al ejecutar la query");
    for _carousel in carousel_result {
        println!(
            "
        id_carousel:{} 
        title_carousel_1:{}
        title_carousel_2:{}
        title_carousel_3:{}
        p_carousel_1:{}
        p_carousel_2:{} 
        p_carousel_3:{}  
        img_carousel_1:{}
        img_carousel_2:{}
        img_carousel_3:{}
        href_carousel_1:{}
        href_carousel_2:{} 
        href_carousel_3:{}  
        ",
            _carousel.id_carousel,
            _carousel.title_carousel_1,
            _carousel.title_carousel_2,
            _carousel.title_carousel_3,
            _carousel.p_carousel_1,
            _carousel.p_carousel_2,
            _carousel.p_carousel_3,
            _carousel.img_carousel_1,
            _carousel.img_carousel_2,
            _carousel.img_carousel_3,
            _carousel.href_carousel_1,
            _carousel.href_carousel_2,
            _carousel.href_carousel_3
        );
    }
}
//Final Funcion Seleccionar todos los Carousel

//Start Funcion Seleccionar todos los Symbol
pub fn query_select_all_symbol() {
    println!("Query Select All Initial Symbol");
    let mut conn = connect_db(); //Conectamos a la base de datos
                                 //Select * from posts
    let symbol_result = symbol
        .order(id_symbol)
        .load::<Symbol>(&mut conn)
        .expect("Error al ejecutar la query");
    for _symbol in symbol_result {
        println!(
            "
        id_symbol:{} 
        name_symbol:{}
        description_symbol:{}
        img_symbol:{}
        href_symbol:{}
        ",
            _symbol.id_symbol,
            _symbol.title_symbol,
            _symbol.p_symbol,
            _symbol.img_symbol,
            _symbol.href_symbol
        );
    }
}
//Final Funcion Seleccionar todos los Symbol

//Start Funcion Seleccionar todos los Parrafo index
pub fn query_select_all_p_index() {
    println!("Query Select All Initial P Index");
    let mut conn = connect_db(); //Conectamos a la base de datos
                                 //Select * from posts
    let p_index_result = pindex
        .order(id_p_index)
        .load::<Pindex>(&mut conn)
        .expect("Error al ejecutar la query");
    for _p_index in p_index_result {
        println!(
            "
        id_p_index:{} 
        title_p_index:{}
        sub_title_p_index:{}
        p_p_index_1:{}
        p_p_index_2:{}
        p_p_index_3:{}
        img_p_index:{}
        ",
            _p_index.id_p_index,
            _p_index.title_p_index,
            _p_index.sub_title_p_index,
            _p_index.p_p_index_1,
            _p_index.p_p_index_2,
            _p_index.p_p_index_3,
            _p_index.img_p_index
        );
    }
}
//Final Funcion Seleccionar todos los Parrafo index
//*********************************************End Consultas Select Template Index*********************************************

//*********************************************Consultas Select Template Service*********************************************
//Start Funcion Seleccionar todos los Service
pub fn query_select_all_service() {
    println!("Query Select All Initial Service");
    let mut conn = connect_db(); //Conectamos a la base de datos

    //Select * from posts
    let service_result = services
        .order(id_service)
        .load::<Service>(&mut conn)
        .expect("Error al ejecutar la query");
    for _service in service_result {
        println!(
            "
        Id:{} 
        Name:{}
        Description:{}
        Img Service 1:{}
        Img Service 2:{}
        Img Service 3:{}
        Published:{}",
            _service.id_service,
            _service.name,
            _service.description,
            _service.img_service_1,
            _service.img_service_2,
            _service.img_service_3,
            _service.published_service
        );
    }
}
//Final Funcion Seleccionar todos los Service

//Start Funcion Seleccionar todos los Parrafo Service
pub fn query_select_all_p_service() {
    println!("Query Select All Initial P Service");
    let mut conn = connect_db(); //Conectamos a la base de datos

    //Select * from posts
    let p_service_result = pservice
        .order(id_p_service)
        .load::<Pservice>(&mut conn)
        .expect("Error al ejecutar la query");
    for _p_service in p_service_result {
        println!(
            "
        Id:{} 
        title_p_service:{}
        sub_title_p_service:{}
        p_p_service_1 :{}
        p_p_service_2 :{}
        p_p_service_3 :{}
        img_p_service:{}",
            _p_service.id_p_service,
            _p_service.title_p_service,
            _p_service.sub_title_p_service,
            _p_service.p_p_service_1,
            _p_service.p_p_service_2,
            _p_service.p_p_service_3,
            _p_service.img_p_service
        );
    }
}
//Final Funcion Seleccionar todos los Parrafo Service

//Start Funcion Seleccionar todos los Layout Service
pub fn query_select_all_layout_service() {
    println!("Query Select All Initial Layout Service");
    let mut conn = connect_db(); //Conectamos a la base de datos

    //Select * from posts
    let layout_service_result = layoutservice
        .order(id_layout_service)
        .load::<LayoutService>(&mut conn)
        .expect("Error al ejecutar la query");
    for _layout_service in layout_service_result {
        println!(
            "
        Id:{} 
        title_layout_service:{}
        p_p_layout_service:{}
        img_layout_service :{}
        title_button_layout_service_1 :{}
        title_button_layout_service_2 :{}
        href_layout_service_1:{}
        href_layout_service_2:{}",
            _layout_service.id_layout_service,
            _layout_service.title_layout_service,
            _layout_service.p_p_layout_service,
            _layout_service.img_layout_service,
            _layout_service.title_button_layout_service_1,
            _layout_service.title_button_layout_service_2,
            _layout_service.href_layout_service_1,
            _layout_service.href_layout_service_2
        );
    }
}
//Final Funcion Seleccionar todos los Layout Service
//*********************************************End Consultas Select Template Service*********************************************

//*********************************************Consultas Select Template Profile*********************************************
//Start Funcion Seleccionar todos los Layout Profile
pub fn query_select_all_layout_profile() {
    println!("Query Select All Initial Layout Service");
    let mut conn = connect_db(); //Conectamos a la base de datos

    //Select * from posts
    let _layout_profile_result = layoutprofile
        .order(id_layout_profile)
        .load::<LayoutProfile>(&mut conn)
        .expect("Error al ejecutar la query");
    for _layout_profile in _layout_profile_result {
        println!(
            "
        Id:{} 
        title_layout_service:{}
        p_p_layout_service:{}
        img_layout_service :{}
        title_button_layout_service_1 :{}
        title_button_layout_service_2 :{}
        href_layout_service_1:{}
        href_layout_service_2:{}",
            _layout_profile.id_layout_profile,
            _layout_profile.title_layout_profile,
            _layout_profile.p_p_layout_profile,
            _layout_profile.img_layout_profile,
            _layout_profile.title_button_layout_profile_1,
            _layout_profile.title_button_layout_profile_2,
            _layout_profile.href_layout_profile_1,
            _layout_profile.href_layout_profile_2
        );
    }
}
//Final Funcion Seleccionar todos los Layout Profile

pub fn query_select_all_p_profile() {
    println!("Query Select All Initial P Profile");
    let mut conn = connect_db(); //Conectamos a la base de datos

    let p_profile_result = pprofile
        .order(id_p_profile)
        .load::<Pprofile>(&mut conn)
        .expect("Error al ejecutar la query");
    for _p_profile in p_profile_result {
        println!(
            "
        Id:{} 
        title_p_profile:{}
        sub_title_p_profile:{}
        p_p_profile_1 :{}
        p_p_profile_2 :{}
        p_p_profile_3 :{}
        img_p_profile:{}",
            _p_profile.id_p_profile,
            _p_profile.title_p_profile,
            _p_profile.sub_title_p_profile,
            _p_profile.p_p_profile_1,
            _p_profile.p_p_profile_2,
            _p_profile.p_p_profile_3,
            _p_profile.img_p_profile
        );
    }
}

pub fn query_select_all_skills() {
    println!("Query Select All Initial Skills");
    let mut conn = connect_db(); //Conectamos a la base de datos

    let skills_result = skills
        .order(id_skill)
        .load::<Skilss>(&mut conn)
        .expect("Error al ejecutar la query");
    for _skills in skills_result {
        println!(
            "
        Id:{} 
        title_skill:{}
        name_skill: {}
        level_skill:{}
        type_skill :{}
        img_skill :{}
        img_perfil_skill:{}
        ",
            _skills.id_skill,
            _skills.title_skill,
            _skills.name_skill,
            _skills.level_skill,
            _skills.type_skill,
            _skills.img_skill,
            _skills.img_perfil_skill
        );
    }
}

//Start Funcion Seleccionar todos los Profile
pub fn query_select_all_profile() {
    println!("Query Select All Initial Profile");
    let mut conn = connect_db(); //Conectamos a la base de datos

    //Select * from posts
    let profile_result = profiles
        .order(id_profile)
        .load::<Profile>(&mut conn)
        .expect("Error al ejecutar la query");
    for _profile in profile_result {
        println!(
            "
        Id:{} 
        Name:{}
        Last Name:{} 
        Email:{} 
        Git Hub:{} 
        Phone:{} 
        Address:{} 
        City:{} 
        Country:{} 
        Text:{} 
        Published:{}",
            _profile.id_profile,
            _profile.name,
            _profile.last_name,
            _profile.email,
            _profile.git_hub,
            _profile.phone,
            _profile.address,
            _profile.city,
            _profile.country,
            _profile.text,
            _profile.published_profile
        );
    }
}
//Final Funcion Seleccionar todos los Profile
//*********************************************End Consultas Select Template Profile*********************************************

//********************************************* Consultas Select Template Post*********************************************
//Start Funcion Seleccionar layout Post
pub fn query_select_all_layout_post() {
    println!("Query Select All Initial Layout Post");
    let mut conn = connect_db(); //Conectamos a la base de datos

    //Select * from posts
    let layout_post_result = layoutpost
        .order(id_layout_post)
        .load::<LayoutPost>(&mut conn)
        .expect("Error al ejecutar la query");
    for _layout_post in layout_post_result {
        println!(
            "
        Id:{} 
        title_layout_post:{}
        p_p_layout_post:{}
        img_layout_post_1 :{}
        img_layout_post_2 :{}
        title_button_layout_post_1:{}
        href_layout_post_1:{}",
            _layout_post.id_layout_post,
            _layout_post.title_layout_post,
            _layout_post.p_p_layout_post,
            _layout_post.img_layout_post_1,
            _layout_post.img_layout_post_2,
            _layout_post.title_button_layout_post_1,
            _layout_post.href_layout_post_1
        );
    }
}

//Start Funcion Seleccionar todos los post
pub fn query_select_all_post() {
    println!("Query Select All Initial Post");
    let mut conn = connect_db(); //Conectamos a la base de datos

    //Select * from posts
    let posts_result = posts
        .order(id)
        .load::<Post>(&mut conn)
        .expect("Error al ejecutar la query");
    for _post in posts_result {
        println!(
            "
        Id:{} 
        Title:{}
        Slug:{} 
        Bode:{}
        Img:{} 
        DAte: {}
        Published:{}",
            _post.id, _post.title, _post.slug, _post.body, _post.img, _post.date, _post.published
        );
    }
}
//Final Funcion Seleccionar todos los post

//Start Funcion Seleccionar todos los Post por Titulo
pub fn query_select_title(_title: &str) {
    println!("Query Select Title Initial");
    let mut conn = connect_db(); //Conectamos a la base de datos

    //Select * from posts where title = "First Post"
    let post_result = posts
        .filter(title.eq(_title))
        .load::<Post>(&mut conn)
        .expect("Error al ejecutar la query");
    for _post in post_result {
        println!(
            "
        Id:{} 
        Title:{}
        Slug:{} 
        Bode:{} 
        Published:{}",
            _post.id, _post.title, _post.slug, _post.body, _post.published
        );
    }
}
//Final Funcion Seleccionar todos los Post por Titulo
//*********************************************End Consultas Select Template Post*********************************************

//*********************************************Consultas Select Template Contact*********************************************
//Start Funcion Seleccionar los parrafos Contact
pub fn query_selct_all_p_contact() {
    println!("Query Select All Initial P Contact");
    let mut conn = connect_db(); //Conectamos a la base de datos

    //Select * from posts
    let p_contact_result = p_contact
        .order(id_p_contact)
        .load::<PContact>(&mut conn)
        .expect("Error al ejecutar la query");
    for _p_contact in p_contact_result {
        println!(
            "
        Id:{} 
        title_p_contact:{}
        p_p_contact_1 :{}
        p_p_contact_2 :{}
        p_p_contact_3 :{}
        title_button_contact_1 :{}
        title_button_contact_2 :{}
        title_button_contact_3 :{}
        href_contact_1 :{}
        href_contact_2 :{}
        href_contact_3:{}",
            _p_contact.id_p_contact,
            _p_contact.title_p_contact,
            _p_contact.p_p_contact_1,
            _p_contact.p_p_contact_2,
            _p_contact.p_p_contact_3,
            _p_contact.title_button_contact_1,
            _p_contact.title_button_contact_2,
            _p_contact.title_button_contact_3,
            _p_contact.href_contact_1,
            _p_contact.href_contact_2,
            _p_contact.href_contact_3
        );
    }
}

//Start Funcion Seleccionar todos los Social
pub fn query_select_all_social() {
    println!("Query Select All Initial Social");
    let mut conn = connect_db(); //Conectamos a la base de datos
                                 //Select * from posts
    let social_result = socials
        .order(id_social)
        .load::<Social>(&mut conn)
        .expect("Error al ejecutar la query");
    for _social in social_result {
        println!(
            "
        Id:{} 
        Name:{}
        Telegram:{}
        Linkedin:{}
        Github:{}
        Instagram:{}
        Whatsapp:{}
        Twitter:{}
        Published:{}",
            _social.id_socials,
            _social.name,
            _social.telegram,
            _social.linkedin,
            _social.github,
            _social.instagram,
            _social.whatsapp,
            _social.twitter,
            _social.published_social
        );
    }
}
//Final Funcion Seleccionar todos los Social

//Start Funcion Seleccionar todos los Msg
pub fn query_selct_all_msg() {
    println!("Query Select All Initial Msg");
    let mut conn = connect_db(); //Conectamos a la base de datos
                                 //Select * from posts
    let msg_result = msg
        .order(first_name)
        .load::<Msg>(&mut conn)
        .expect("Error al ejecutar la query");
    for _msg in msg_result {
        println!(
            "Contact Post 
        
        firstName:{},
        lastName:{},
        email:{},
        country: {},
        message:{},
    ",
            _msg.first_name, _msg.last_name, _msg.email, _msg.country, _msg.message
        );
    }
}
//Final Funcion Seleccionar todos los Msg
//*********************************************End Consultas Select Template Contact*********************************************
