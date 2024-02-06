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

//********************************************* Consultas Insert Template Index*********************************************
//Start Funcion insertar un nuevo Carousel
pub fn query_insert_carousel(
    _title_carousel_1: &str,
    _title_carousel_2: &str,
    _title_carousel_3: &str,
    _p_carousel_1: &str,
    _p_carousel_2: &str,
    _p_carousel_3: &str,
    _img_carousel_1: &str,
    _img_carousel_2: &str,
    _img_carousel_3: &str,
    _href_carousel_1: &str,
    _href_carousel_2: &str,
    _href_carousel_3: &str,
) {
    println!("Query Insert Initial Carousel");
    let mut conn = connect_db();
    // Create a new_carousel
    let new_carousel = NewCarousel {
        title_carousel_1: _title_carousel_1,
        title_carousel_2: _title_carousel_2,
        title_carousel_3: _title_carousel_3,
        p_carousel_1: _p_carousel_1,
        p_carousel_2: _p_carousel_2,
        p_carousel_3: _p_carousel_3,
        img_carousel_1: _img_carousel_1,
        img_carousel_2: _img_carousel_2,
        img_carousel_3: _img_carousel_3,
        href_carousel_1: _href_carousel_1,
        href_carousel_2: _href_carousel_2,
        href_carousel_3: _href_carousel_3,
    };

    let _carousel: Carousel = diesel::insert_into(carousel::table)
        .values(&new_carousel)
        .get_result(&mut conn)
        .expect("Error al insertar el carousel");

    let carousel_result = carousel
        .limit(1)
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
//Final Funcion insertar un nuevo Carousel

//Start Funcion insertar un nuevo Symbol
pub fn query_insert_symbol(
    _title_symbol: &str,
    _p_symbol: &str,
    _img_symbol: &str,
    _href_symbol: &str,
) {
    println!("Query insert symbol");
    let mut conn = connect_db();

    let new_symbol = NewSymbol {
        title_symbol: _title_symbol,
        p_symbol: _p_symbol,
        img_symbol: _img_symbol,
        href_symbol: _href_symbol,
    };

    let _symbol: Symbol = diesel::insert_into(symbol::table)
        .values(new_symbol)
        .get_result(&mut conn)
        .expect("No se ha podido establecer los datos.");

    let symbol_result = symbol
        .limit(1)
        .load::<Symbol>(&mut conn)
        .expect("Error al ejecutar la query");

    for _symbol in symbol_result {
        println!(
            "
        title_symbol :{} 
        p_symbol :{}   
        img_symbol :{}   
        href_symbol :{}      
        
        ",
            _symbol.title_symbol, _symbol.p_symbol, _symbol.img_symbol, _symbol.href_symbol
        );
    }
}
//Final Funcion insertar un nuevo Symbol

//Start Funcion insertar un nuevo Parrafo index
pub fn query_insert_p_index(
    _title_p_index: &str,
    _sub_title_p_index: &str,
    _p_p_index_1: &str,
    _p_p_index_2: &str,
    _p_p_index_3: &str,
    _img_p_index: &str,
) {
    println!("Query Insert Initial P Index");
    let mut conn = connect_db();

    let new_p_index = NewPindex {
        title_p_index: _title_p_index,
        sub_title_p_index: _sub_title_p_index,
        p_p_index_1: _p_p_index_1,
        p_p_index_2: _p_p_index_2,
        p_p_index_3: _p_p_index_3,
        img_p_index: _img_p_index,
    };

    let _p_index: Pindex = diesel::insert_into(pindex::table)
        .values(new_p_index)
        .get_result(&mut conn)
        .expect("No se ha podido establecer los datos.");

    let p_index_result = pindex
        .limit(1)
        .load::<Pindex>(&mut conn)
        .expect("Error al ejecutar la query");

    for _p_index in p_index_result {
        println!(
            "
        title_p_index:{}
        sub_title_p_index:{}
        p_p_index_1:{}
        p_p_index_2:{}
        p_p_index_3:{}
        img_p_index:{}
        ",
            _p_index.title_p_index,
            _p_index.sub_title_p_index,
            _p_index.p_p_index_1,
            _p_index.p_p_index_2,
            _p_index.p_p_index_3,
            _p_index.img_p_index
        );
    }
}
//Final Funcion insertar un nuevo Parrafo index
//********************************************* End Consultas Insert Template Index*********************************************

//*********************************************Consultas Insert Template Service*********************************************
//Start Funcion insertar un nuevo Service
pub fn query_insert_service(
    _name: &str,
    _description: &str,
    _img_service_1: &str,
    _img_service_2: &str,
    _img_service_3: &str,
    _published_service: bool,
) {
    println!("Query Insert Initial Service");
    let mut conn = connect_db();

    let new_service = NewService {
        name: _name,
        description: _description,
        img_service_1: _img_service_1,
        img_service_2: _img_service_2,
        img_service_3: _img_service_3,
        published_service: _published_service,
    };

    let _service: Service = diesel::insert_into(services::table)
        .values(&new_service)
        .get_result(&mut conn)
        .expect("Error al insertar el service");

    let service_result = services
        .limit(1)
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
//Final Funcion insertar un nuevo Service

//Start Funcion insertar un nuevo Parrafo Service
pub fn query_insert_p_service(
    _title_p_service: &str,
    _sub_title_p_service: &str,
    _p_p_service_1: &str,
    _p_p_service_2: &str,
    _p_p_service_3: &str,
    _img_p_service: &str,
) {
    println!("Query Insert Initial Parrafo Service");
    let mut conn = connect_db();

    let new_p_service = NewParrafoService {
        title_p_service: _title_p_service,
        sub_title_p_service: _sub_title_p_service,
        p_p_service_1: _p_p_service_1,
        p_p_service_2: _p_p_service_2,
        p_p_service_3: _p_p_service_3,
        img_p_service: _img_p_service,
    };

    let _p_service: Pservice = diesel::insert_into(pservice::table)
        .values(&new_p_service)
        .get_result(&mut conn)
        .expect("Error al insertar el parrafo service");

    let p_service_result = pservice
        .limit(1)
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
//Final Funcion insertar un nuevo Parrafo Service

//Start Funcion insertar un nuevo Layout Service
pub fn query_insert_layout_service(
    _title_layout_service: &str,
    _p_p_layout_service: &str,
    _img_layout_service: &str,
    _title_button_layout_service_1: &str,
    _title_button_layout_service_2: &str,
    _href_layout_service_1: &str,
    _href_layout_service_2: &str,
) {
    println!("Query Insert Initial Layout Service");
    let mut conn = connect_db();

    let new_layout_service = NewLayoutService {
        title_layout_service: _title_layout_service,
        p_p_layout_service: _p_p_layout_service,
        img_layout_service: _img_layout_service,
        title_button_layout_service_1: _title_button_layout_service_1,
        title_button_layout_service_2: _title_button_layout_service_2,
        href_layout_service_1: _href_layout_service_1,
        href_layout_service_2: _href_layout_service_2,
    };

    let _layout_service: LayoutService = diesel::insert_into(layoutservice::table)
        .values(&new_layout_service)
        .get_result(&mut conn)
        .expect("Error al insertar el layout service");

    let layout_service_result = layoutservice
        .limit(1)
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
//Final Funcion insertar un nuevo Layout Service
//*********************************************End Consultas Insert Template Service*********************************************

//*********************************************Consultas Insert Template Profile*********************************************
pub fn query_insert_layout_profile(
    _title_layout_profile: &str,
    _p_p_layout_profile: &str,
    _img_layout_profile: &str,
    _title_button_layout_profile_1: &str,
    _title_button_layout_profile_2: &str,
    _href_layout_profile_1: &str,
    _href_layout_profile_2: &str,
) {
    println!("Query Insert Initial Layout Profile");
    let mut conn = connect_db();

    let new_layout_profile = NewLayoutprofile {
        title_layout_profile: _title_layout_profile,
        p_p_layout_profile: _p_p_layout_profile,
        img_layout_profile: _img_layout_profile,
        title_button_layout_profile_1: _title_button_layout_profile_1,
        title_button_layout_profile_2: _title_button_layout_profile_2,
        href_layout_profile_1: _href_layout_profile_1,
        href_layout_profile_2: _href_layout_profile_2,
    };

    let _layout_profile: LayoutProfile = diesel::insert_into(layoutprofile::table)
        .values(&new_layout_profile)
        .get_result(&mut conn)
        .expect("Error al insertar el layout profile");

    let layout_profile_result = layoutprofile
        .limit(1)
        .load::<LayoutProfile>(&mut conn)
        .expect("Error al ejecutar la query");

    for _layout_profile in layout_profile_result {
        println!(
            "
            Id:{} 
            title_layout_profile:{}
            p_p_layout_profile:{}
            img_layout_profile :{}
            title_button_layout_profile_1 :{}
            title_button_layout_profile_2 :{}
            href_layout_profile_1:{}
            href_layout_profile_2:{}",
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

pub fn query_insert_p_profile(
    _title_p_profile: &str,
    _sub_title_p_profile: &str,
    _p_p_profile_1: &str,
    _p_p_profile_2: &str,
    _p_p_profile_3: &str,
    _img_p_profile: &str,
) {
    println!("Query Insert Initial Parrafo Profile");
    let mut conn = connect_db();

    let new_p_profile = NewParrafoProfile {
        title_p_profile: _title_p_profile,
        sub_title_p_profile: _sub_title_p_profile,
        p_p_profile_1: _p_p_profile_1,
        p_p_profile_2: _p_p_profile_2,
        p_p_profile_3: _p_p_profile_3,
        img_p_profile: _img_p_profile,
    };

    let _p_profile: Pprofile = diesel::insert_into(pprofile::table)
        .values(&new_p_profile)
        .get_result(&mut conn)
        .expect("Error al insertar el parrafo profile");

    let p_profile_result = pprofile
        .limit(1)
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

//Start Funcion insertar un nuevo profile
pub fn query_insert_profile(
    _name: &str,
    _last_name: &str,
    _email: &str,
    _git_hub: &str,
    _phone: &str,
    _address: &str,
    _city: &str,
    _country: &str,
    _text: &str,
    _published_profile: bool,
) {
    println!("Query Insert Initial Profile");
    let mut conn = connect_db();

    let new_profile = NewProfile {
        name: _name,
        last_name: _last_name,
        email: _email,
        git_hub: _git_hub,
        phone: _phone,
        address: _address,
        city: _city,
        country: _country,
        text: _text,
        published_profile: &_published_profile,
    };

    let _profile: Profile = diesel::insert_into(profiles::table)
        .values(&new_profile)
        .get_result(&mut conn)
        .expect("Error al insertar el profile");

    let profile_result = profiles
        .limit(1)
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
//Final Funcion insertar un nuevo profile

pub fn query_insert_skills(
    _name_skill: &str,
    _level_skill: &str,
    _type_skill: &str,
    _img_skill: &str,
) {
    println!("Query Insert Initial Skills");
    let mut conn = connect_db();
    let tittle: &str = "Skills";
    let img_profile_white_hacker =
        "https://img2.freepng.es/20180402/lhe/kisspng-white-hat-security-hacker-anonymous-clip-art-anonymous-5ac1b1ffbc2056.6923529915226434557706.jpg";

    let new_skill = NewSkill {
        title_skill: tittle,
        name_skill: _name_skill,
        level_skill: _level_skill,
        type_skill: _type_skill,
        img_skill: _img_skill,
        img_perfil_skill: img_profile_white_hacker,
    };

    let _skill: Skilss = diesel::insert_into(skills::table)
        .values(&new_skill)
        .get_result(&mut conn)
        .expect("Error al insertar el skill");

    let skill_result = skills
        .limit(1)
        .load::<Skilss>(&mut conn)
        .expect("Error al ejecutar la query");

    for _skill in skill_result {
        println!(
            "
            Id:{} 
            title_skill:{}
            name_skill:{}
            level_skill :{}
            type_skill :{}
            img_skill :{}
            img_perfil_skill:{}",
            _skill.id_skill,
            _skill.title_skill,
            _skill.name_skill,
            _skill.level_skill,
            _skill.type_skill,
            _skill.img_skill,
            _skill.img_perfil_skill
        );
    }
}

//*********************************************End Consultas Insert Template Profile*********************************************

//********************************************* Consultas Insert Template post*********************************************
//Start Funcion insertar un layout post
pub fn query_insert_layout_post(
    _title_layout_post: &str,
    _p_p_layout_post: &str,
    _img_layout_post_1: &str,
    _img_layout_post_2: &str,
    _title_button_layout_post_1: &str,
    _href_layout_post_1: &str,
) {
    println!("Query Insert Initial Layout Post");
    let mut conn = connect_db();

    let new_layout_post = NewLayoutpost {
        title_layout_post: _title_layout_post,
        p_p_layout_post: _p_p_layout_post,
        img_layout_post_1: _img_layout_post_1,
        img_layout_post_2: _img_layout_post_2,
        title_button_layout_post_1: _title_button_layout_post_1,
        href_layout_post_1: _href_layout_post_1,
    };

    let _layout_post: LayoutPost = diesel::insert_into(layoutpost::table)
        .values(&new_layout_post)
        .get_result(&mut conn)
        .expect("Error al insertar el layout post");

    let layout_post_result = layoutpost
        .limit(1)
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
        title_button_layout_post_1 :{}
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

//Start Funcion insertar un nuevo post
pub fn query_insert_post(_title: &str, _body: &str, _slug: &str, _img: &str, _published: bool) {
    //Time
    use chrono::DateTime;
    use chrono::Local;
    let local_date: DateTime<Local> = Local::now();
    let string_date = local_date.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("Local Date: {}", string_date);

    println!("Query Insert Initial");
    let mut conn = connect_db();

    let new_post = NewPost {
        title: _title,
        slug: _slug,
        body: _body,
        img: _img,
        date: &string_date,
        published: &_published,
    };

    let _post: Post = diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(&mut conn)
        .expect("Error al insertar el post");

    let post_result = posts
        .limit(1)
        .load::<Post>(&mut conn)
        .expect("Error al ejecutar la query");

    for _post in post_result {
        println!(
            "
        Id:{} 
        Title:{}
        Slug:{} 
        Bode:{} 
        Img:{}
        Date:{}
        Published:{}",
            _post.id, _post.title, _post.slug, _post.body, _post.img, _post.date, _post.published
        );
    }
}
//Final Funcion insertar un nuevo post

//Start Funcion update un Post
pub fn query_update_or_edit(_id: i32, _title: &str, _slug: &str, _body: &str, _published: bool) {
    println!("Query Update or Edit Initial");
    let mut conn = connect_db();

    let _post: Post = diesel::update(posts.filter(id.eq(_id)))
        .set((
            title.eq(_title),
            slug.eq(_slug),
            body.eq(_body),
            published.eq(_published),
        ))
        .get_result(&mut conn)
        .expect("Error al ejecutar la query");

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
//Final Funcion update un Post

//Start Funcion delete un Post
pub fn query_delete(_id: i32) {
    println!("Query Delete Initial");
    let mut conn = connect_db();

    let _post: Post = diesel::delete(posts.filter(id.eq(_id)))
        .get_result(&mut conn)
        .expect("Error al ejecutar la query");

    println!("Post Delete: {}", _post.id);
}
//Final Funcion delete un Post
//********************************************* END Consultas Insert Template post*********************************************

//********************************************* Consultas Insert Template Contact*********************************************
//Start Funcion insertar un parrafo contacto
pub fn query_insert_p_contact(
    _title_p_contact: &str,
    _p_p_contact_1: &str,
    _p_p_contact_2: &str,
    _p_p_contact_3: &str,
    _title_button_contact_1: &str,
    _title_button_contact_2: &str,
    _title_button_contact_3: &str,
    _href_contact_1: &str,
    _href_contact_2: &str,
    _href_contact_3: &str,
) {
    println!("Query Insert Initial P Contact");
    let mut conn = connect_db();

    let new_p_contact = NewPcontact {
        title_p_contact: _title_p_contact,
        p_p_contact_1: _p_p_contact_1,
        p_p_contact_2: _p_p_contact_2,
        p_p_contact_3: _p_p_contact_3,
        title_button_contact_1: _title_button_contact_1,
        title_button_contact_2: _title_button_contact_2,
        title_button_contact_3: _title_button_contact_3,
        href_contact_1: _href_contact_1,
        href_contact_2: _href_contact_2,
        href_contact_3: _href_contact_3,
    };

    let _p_contact: PContact = diesel::insert_into(p_contact::table)
        .values(&new_p_contact)
        .get_result(&mut conn)
        .expect("Error al insertar el p contact");

    let p_contact_result = p_contact
        .limit(1)
        .load::<PContact>(&mut conn)
        .expect("Error al ejecutar la query");

    for _p_contact in p_contact_result {
        println!(
            "
        Id:{} 
        title_p_contact:{}
        p_p_contact_1:{}
        p_p_contact_2:{}
        p_p_contact_3:{}
        title_button_contact_1:{}
        title_button_contact_2:{}
        title_button_contact_3:{}
        href_contact_1:{}
        href_contact_2:{}
        href_contact_3:{}
        ",
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

//Start Funcion insertar un nuevo Redes sociales
pub fn query_insert_social(
    _name: &str,
    _telegram: &str,
    _linkedin: &str,
    _github: &str,
    _instagram: &str,
    _whatsapp: &str,
    _twitter: &str,
    _published_social: bool,
) {
    println!("Query Insert Initial Social");
    let mut conn = connect_db();

    let new_social = NewSocial {
        name: _name,
        telegram: _telegram,
        linkedin: _linkedin,
        github: _github,
        instagram: _instagram,
        whatsapp: _whatsapp,
        twitter: _twitter,
        published_social: _published_social,
    };

    let _social: Social = diesel::insert_into(socials::table)
        .values(&new_social)
        .get_result(&mut conn)
        .expect("Error al insertar el social");

    let social_result = socials
        .limit(1)
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
//Final Funcion insertar un nuevo Redes sociales

//Start Funcion insertar un nuevo Msg
pub fn query_insert_msg(
    _first_name: &str,
    _last_name: &str,
    _email: &str,
    _country: &str,
    _message: &str,
) {
    println!("Query Insert Initial Msg");
    let mut conn = connect_db();

    let new_msg = NewMsg {
        first_name: _first_name,
        last_name: _last_name,
        email: _email,
        country: _country,
        message: _message,
    };

    let _msg: Msg = diesel::insert_into(msg::table)
        .values(&new_msg)
        .get_result(&mut conn)
        .expect("Error al insertar el msg");

    let msg_result = msg
        .limit(1)
        .load::<Msg>(&mut conn)
        .expect("Error al ejecutar la query");
    for _msg in msg_result {
        println!(
            "
        First Name:{}
        Last Name:{}
        Email:{}
        Country:{}
        Message:{}
        ",
            _msg.first_name, _msg.last_name, _msg.email, _msg.country, _msg.message
        );
    }
}
//Final Funcion insertar un nuevo Msg
//*********************************************End Consultas Insert Template Contact*********************************************
