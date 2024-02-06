#[macro_use] //Para usar las macro creada
extern crate diesel; //Extendible de diesel
extern crate tera; //Extendible de tera

pub mod api; //Usando los datos de la api
pub mod database; //Usando los datos de la base de datos
pub mod inserts; //Usando los datos de los inserts
pub mod models; //Usando los datos de los modelos
pub mod schemas; //Usando los datos del squema //Usando los datos de la api

use crate::api::{
    contact, contact_post, index, post, profile, query_index_get, query_index_post, service,
}; //Usando los datos de la api
use actix_web::{web, App, HttpServer}; //Usando Actix web
use diesel::pg::PgConnection; // Import the Diesel postgres connection
use diesel::r2d2::Pool; //Import the r2d2 pool multi conexiones para la data base
use diesel::r2d2::{self, ConnectionManager}; //Import the r2d2 connection manager
use dotenv::dotenv; // Import the dotenv library
use std::env; //Import the env library
use tera::Tera; //Usando Tera

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>; //Definir el tipo de la conexion

//********************************************* PRINCIPAL FUNCTION MAIN*********************************************
#[actix_web::main]
//Funcion  principal
async fn main() -> std::io::Result<()> {
    // Cargar las variables de entorno
    dotenv().ok();
    // Crear la pool de conexiones
    let db_url = env::var("DATABASE_URL").expect("db url variable no encontrada");
    let connection = ConnectionManager::<PgConnection>::new(db_url);
    let pool = Pool::builder()
        .build(connection)
        .expect("No se pudo construir la Pool");

    // Obtener el puerto del servidor
    let port = env::var("PORT").expect("La variable de entorno PORT no existe.");
    let port: u16 = port.parse().unwrap();

    // Print the url of the server to the console
    println!("Server running at http://localhost:{}", port);

    //Crear el servidor
    HttpServer::new(move || {
        //Use Tera
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        
        //Crear el servidor
        App::new()
            .service(index)
            .service(query_index_get) //para que no hayan errores en la pool de los datos se deben de crear una copia  de los datos y inicializarla en el servidor con una nueva
            .service(query_index_post)
            .service(profile)
            .service(service)
            .service(post)
            .service(contact)
            .service(contact_post)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(tera.clone()))
    })
    .bind(("0.0.0.0", port))
    .unwrap()
    .run()
    .await
}
//********************************************* END PRINCIPAL FUNCTION MAIN*********************************************

/*
TIPS PARA LOS COMENTARIOS PARA DOCUMENTAR MY CODE DIOSSSS TODO LO QUE FALTA POR HACER "THANKS GOOD , FOR THE GOOD TIMES IN LIVE CODING "
 *aaaa
 !aaaa
 ? aaaa
 TODO : aa
 @param MyParam
*/
