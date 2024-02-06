### blog-juan

## Create project with Rust and Cargo

```bash
cargo new blog-juan
```

## Run project

```bash
cargo run
```

## Build project

```bash
cargo build
```

## Install diesel_cli

```bash
cargo install diesel_cli --no-default-features --features postgres
```

## Run migration with diesel

```bash
diesel migration run
```

## Create migration with diesel

```bash
diesel migration generate create_posts
```

## Setup for diesel

```bash
diesel setup
```

## Password database

sHiscL1Uh7mLfZfM

## Create .env file

```bash
touch .env
```

## Add environment variables to .env file

```bash
DATABASE_URL=postgres://juan:xxxxxxxx@xxxxxxxxxxxxxxxxxxx
PORT=3000
```

## Queries for select data

```bash
    //Funciones para consultar en la base de datos
    //index
    query_select_all_carousel();
    query_select_all_symbol();
    query_select_all_p_index();
    //Services
    query_select_all_layout_service();
    query_select_all_p_service();
    query_select_all_service();
    //Profile
    query_select_all_layout_profile();
    query_select_all_p_profile();
    query_select_all_profile();
    query_select_all_skills();
    //Post
    query_select_all_layout_post();
    query_select_all_post();
    //Contact
    query_selct_all_p_contact();
    query_select_all_social();
    query_selct_all_msg();
```

## Queries insert data for page

```bash
      //Funciones para insertar en la base de datos
      //index
      //Insertar carousel #1
      query_insert_carousel(
          "Full Stack Developer",
          "Where I study?",
          "I increase my skills in platzi",
          "I am autonomous in my personal development as a systems engineer",
          "I am a student of systems engineering at the University of San Buenaventura de Cali and I am studying 5 semester",
          "With platzi I acquire new soft skills in order to improve my profile",
          "https://cdn.pixabay.com/photo/2022/04/06/04/47/ai-7114792__340.jpg",
          "https://cdn.discordapp.com/attachments/410558873951404053/1019345002398228600/carousel_u.jpg",
          "https://cdn.discordapp.com/attachments/410558873951404053/1019346398161940530/garfield-programmer.jpg",
          "/contact",
          "https://www.usbcali.edu.co/",
          "https://platzi.com/p/thenowrock/");

      //Insertar symbols #3
      //#1
      query_insert_symbol(
          "Udemy",
          "udemy is another of the platforms on which I keep updating to improve my skills as a developer.",
          "https://cdn.discordapp.com/attachments/410558873951404053/1016491415792001165/udemy-img-x200.png",
          "https://www.udemy.com/user/david-jimenez-190/");
      //#2
      query_insert_symbol(
          "IBM",
          "ibm is the pioneer company in computers and offers us a lot of knowledge for the creation and development of software.",
          "https://cdn.discordapp.com/attachments/410558873951404053/1016492815217332285/ibm2.png",
          "https://www.ibm.com/co-es");
      //#3
      query_insert_symbol(
          "Rust",
          "this website has been developed thanks to the implementation of the rust language for web development or web assembly",
          "https://cdn.discordapp.com/attachments/410558873951404053/1016491415582294076/rust-img-x200.png",
          "https://www.rust-lang.org/");

      //Insertar parrafos en index #3
      //#1
      query_insert_p_index(
          "GROWING TECHNOLOGIES.",
          "New Era 3.0",
          " Since my professional growth and in the ICT area I have seen a great development in the use of new technologies that improve the quality of software development through the following tools that are changing the technological world as we know it: RUST, BOOTSTRAP 5, APIS, DATA STRUCTURES, DATABASES, WEB APPS.",
          "The arrival of the new era of the web, what we mean by this is that we see new development alternatives such as web 3.0, which consists of decentralized technology developments, cryptocurrencies, digital wallets, crypto exchange networks, metaverses, nfts.",
          "That is why I seek to prepare myself and satisfy the new technological needs that the world acclaims that you learn.",
          "https://cdn.discordapp.com/attachments/410558873951404053/1016505600496119878/web1.png",
      );
      //#2
      query_insert_p_index(
          "How we work? ",
          "Agile methodologies",
          "Scrum is a management process that reduces complexity in product development to meet customer needs. Management and Scrum teams work together around requirements and technologies to deliver incrementally working products using empiricism.",
          "Scrum is a simple framework that promotes team collaboration to achieve complex product development.",
          "",
          "https://cdn.discordapp.com/attachments/410558873951404053/1016498990138798122/scrum1.png",
      );
      //#3
      query_insert_p_index(
          "Goals when working.",
          "Productivity",
          " In order to generate quality software, we write code and create projects based on designs, standards and agile methodologies to meet the expectations of our clients, in addition to guaranteeing their quality.",
          "",
          "",
          "https://cdn.discordapp.com/attachments/410558873951404053/1016501971827576903/productivdad1.png",
      );
      //Services
      //Insertar layout service #1
      //#1
      query_insert_layout_service(
          "Services Full Stack",
          "We are interested in creating high quality software products with security standards, optimization, testing, agile methodologies, tdd, scrum, ux.
          We work remotely from the country of Colombia",
          "http://webeys.com/wp-content/uploads/2022/08/fullstack-development-services-1585821765-5355051.png",
          "Development",
          "Contact",
          "/",
          "/contact",
      );
      //Insertar parrafo service #1
      //#1
      query_insert_p_service(
          "My Services.",
          "Profesional Services",
          "My service is based on productivity and delivery of results with agile practices for high-reach results.",
          "I have experience in software development as a systems engineering student, I have high knowledge to develop applications, desktop programs, web apps, APIs, interfaces, tests and mobile services.",
          "I have experience in managing and creating relational databases with postgreSQL, MySQL, such as managing queries and triggers with SQL. As non-relational with MongoDB I also handle FireBase.",
          "https://www.ngeeks.com/wp-content/uploads/2021/09/cursos-programacion.jpg",
      );
      //Insertar service #3
      //#1
      query_insert_service(
          "FRONT-END",
          "Html5,CSS,Bootstrap,JavaScript,React,Angular,Actix-Web",
          "https://www.kampuskod.com/wp-content/uploads/2020/12/frontend-teknolojileri.png",
          "https://i.ytimg.com/vi/NWRU6P6NFsc/maxresdefault.jpg",
          "https://actix.rs/img/jumbotron.jpg",
          true,
      );
      //#2
      query_insert_service(
          "BACK-END",
          "C++,Rust,Python,NodeJS,Java,Git,GitHub",
          "https://luyenkimmau.com.vn/backend-developer-la-gi/imager_9023.jpg",
          "https://blog.facialix.com/wp-content/uploads/2022/02/5fd3903b41d20bd2244ec3fd_programminglanguagesstickers.jpg",
          "https://bs-uploads.toptal.io/blackfish-uploads/components/blog_post_page/content/cover_image_file/cover_image/907884/retina_1708x683_cover-0816-C__Mistakes-Waldek_Newsletter-8ca5fb6eacb673aaad1fe4bfaf4ce205-2f26062f759e8698edd8c5d77b82b992.png",
          true,
      );
      //#3
      query_insert_service(
          "DATA-BASES",
          "MongoDB,PostgreSQL,MySQL,FireBase",
          "https://vietnix.vn/wp-content/uploads/2022/07/postgresql-la-gi-1024x536.webp",
          "https://tecnoticias.net/wp-content/uploads/2021/02/mongodb-atlas-google-cloud-partnership-nosql-databases-integrations-2.jpg",
          "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcStOSAYpLf1bA4JobhA6raFhSG-yHV70U7wAbVsp3jw2FTaVuvMVDVGTrfp8qHFHK-Uasw&usqp=CAU",
          true,
      );

      //Profile
      //Insertar layout profile #1
      query_insert_layout_profile(
          "Development Jobs",
          "Quickly design and customize responsive mobile-first sites with Bootstrap, the world’s most popular front-end open source toolkit, featuring Sass variables and mixins, responsive grid system, extensive prebuilt components, and powerful JavaScript plugins.",
          "https://images.pexels.com/photos/340152/pexels-photo-340152.jpeg?auto=compress&cs=tinysrgb&w=1600",
          "Post",
          "Contact",
          "/post",
          "/contact",
      );
      //Insertar parrafo profile #1
      query_insert_p_profile(
          "My Profile.",
          "Studen",
          "I am currently a systems engineering student at the University of San Buenaventura. I have experience in managing both front-end and back-end technologies.",
          " I don't have previous experience in a June full stack developer job, but my goal is to get that experience with a company that gives me an opportunity.",
          "",
          "https://media.discordapp.net/attachments/410558873951404053/1016533189990301767/apps2.png ",
      );
      //Insertar  profile #1
      query_insert_profile(
          "Juan David",
          "Jimenez",
          "thenowrock@gmail.com",
          "https://github.com/thenowrrock",
          "+57 3178178423",
          "Santa Elena - Cali",
          "Cali",
          "Colombia",
          "hello how am i the developer juan david nice to be able to connect with you",
          true);

      //Insertar skilss profile #1
        query_insert_skills(
            "Rust",
            "Mid",
            "Hybrid",
            "https://jagonzalez.org/wp-content/uploads/2022/08/Diesel-vs-SQLx-interacting-databases-Rust.png",
        );

        //C++
        query_insert_skills(
            "C++",
            "Mid",
            "Hybrid",
            "https://cdn.discordapp.com/attachments/410558873951404053/1019359130210799697/c.jpeg",
        );

        //Python
        query_insert_skills(
            "Python",
            "Mid",
            "Full",
            "https://cdn.discordapp.com/attachments/410558873951404053/1019359219683704902/python.jpg",
        );

        //JavaScript
        query_insert_skills(
            "JavaScript",
            "Mid",
            "Front-Back",
            "https://cdn.discordapp.com/attachments/410558873951404053/1019359238813929493/javascript.jpg",
        );

        //Java
        query_insert_skills(
            "Java",
            "Mid",
            "Full",
            "https://cdn.discordapp.com/attachments/410558873951404053/1019359271663706203/java.jpg",
        );

        //GitHub
        query_insert_skills(
            "GitHub",
            "Mid",
            "Repositoris",
            "https://cdn.discordapp.com/attachments/410558873951404053/1019359326453899264/gihub.png",
        );

        //NodeJs
        query_insert_skills(
            "NodeJs",
            "Nov",
            "Hybrid",
            "https://cdn.discordapp.com/attachments/410558873951404053/1019359370661867622/nodejs.png",
        );

        //Angular
        query_insert_skills(
            "Angular",
            "Mid",
            "Framework",
            "https://cdn.discordapp.com/attachments/410558873951404053/1019359395257266257/angular.png",
        );

        //Raeact
        query_insert_skills(
            "Raeact",
            "Mid",
            "Framework",
            "https://cdn.discordapp.com/attachments/410558873951404053/1019359444083155084/react.png",
        );

        //Html5
        query_insert_skills(
            "Html5",
            "Full",
            "Front",
            "https://cdn.discordapp.com/attachments/410558873951404053/1019359485841657876/html.png",
        );

        //Css3
        query_insert_skills(
            "Css3",
            "Mid",
            "Front",
            "https://cdn.discordapp.com/attachments/410558873951404053/1019359516418134156/css.jpg",
        );

        //Bootstrap
        query_insert_skills(
            "Bootstrap",
            "Mid",
            "Framework",
            "https://cdn.discordapp.com/attachments/410558873951404053/1019359561792102451/bootstrap.png",
        );

        //Actix-Web
        query_insert_skills(
            "Actix-Web",
            "Mid",
            "Framework",
            "https://cdn.discordapp.com/attachments/410558873951404053/1019359579257192538/Actix-Web.png",
        );

        //SQL
        query_insert_skills(
            "SQL",
            "Mid",
            "Back",
            "https://cdn.discordapp.com/attachments/410558873951404053/1019359597775040522/sql.png",
        );

        //PostgreSQL
        query_insert_skills(
            "PostgreSQL",
            "Mid",
            "Database",
            "https://cdn.discordapp.com/attachments/410558873951404053/1019359623813279764/postgresql.jpg",
        );

        //FireBase
        query_insert_skills(
            "FireBase",
            "Nov",
            "Database",
            "https://cdn.discordapp.com/attachments/410558873951404053/1019360919056285787/firebase.jpg",
        );

        //MongoDB
        query_insert_skills(
            "MongoDB",
            "Nov",
            "Database",
            "https://cdn.discordapp.com/attachments/410558873951404053/1019359885953089566/mongodb.png",
        );


        //Insert layout post #1
            query_insert_layout_post(
            "My post",
            "This is my publication space for some personal works and projects in which I have worked and applied what I have learned.",
            "https://www.locurainformaticadigital.com/wp-content/uploads/2017/03/stack-overflow.png",
            "https://www.hispatecno.net/wp-content/uploads/2020/11/1-qtazbwhg6xwe-dhwfiqfma.png",
            "Coming soon",
            "/",
        );

        //Insertar post #1
        query_insert_post(
            "Game role",
            "link de game role",
            "Game role in rust",
            "https://cdn.discordapp.com/attachments/410558873951404053/1019004439106965514/icon.png",
            true,
        );

            //Insertar post #2
        query_insert_post(
            "Create Simple Calculate JS",
            "https://github.com/thenowrrock/calculadora_JS",
            "Calculadora Js",
            "https://i.ytimg.com/vi/f18O80XRe9Q/maxresdefault.jpg",
            true,
        );

        //Insertar post #3
        query_insert_post(
            "Create Simple Calculate Rust",
            "https://github.com/thenowrrock/calculadora-cientifica-rust",
            "Calculadora Rust",
            "https://play-lh.googleusercontent.com/r1XrRuJ30EPw4sOM0JjP0lV9yGNqnYejlRf_C6mNLu15lTcidSAEV4XB3XBhejpJQw",
            true,
        );

        //Insertar post #4
        query_insert_post(
            "Market place react-v1",
            "https://github.com/thenowrrock/market-place-react-v1",
            "Market Place react",
            "https://github.com/thenowrrock/market-place-react-v1/blob/main/Screenshot%202022-08-16%20161138.png?raw=true",
            true,
        );

        //Insertar post #5
        query_insert_post(
            "Sitioweb angular ",
            "https://github.com/thenowrrock/sitioweb-angular",
            "angular Place ",
            "https://github.com/thenowrrock/sitioweb-angular/blob/main/login.png?raw=true",
            true,
        );

        //Insertar post #6
        query_insert_post(
            "Game-role Rust ",
            "https://github.com/thenowrrock/game-role-rust",
            "Game role rs ",
            "https://miro.medium.com/max/709/1*DicTpPZN_m4qMeGCToPW6g.png",
            true,
        );


        //Insertar parrafos contact #1
            query_insert_p_contact(
            "Can you find me ",
            "I am a daily user of the internet since it allows me to communicate how to learn from the comfort of my home, as well as with the profiles of some social networks so that you can contact me and see my personal development as a full stack.",
            "Bmx fresstyle",
            "Colombia is a beautiful country in which we are full of entrepreneurs and technology enthusiasts",
            "Msg",
            "Bmx notice",
            "Colombia notice",
            "/",
            "https://www.instagram.com/reel/ChvIho8pZkw/?utm_source=ig_web_copy_link",
            "https://colombiareports.com/colombia-recommends-stricter-covid-19-measures-for-air-travelers/",
            );

            query_insert_social(
            "Social Juan",
            "https://t.me/+FjUS1UH8AoRkMzAx",
            "https://www.linkedin.com/in/jd-giz-7768a2222/",
            "https://github.com/thenowrrock",
            "https://www.instagram.com/poca_lith/",
            "https://chat.whatsapp.com/JDxfAdCrhusGnUfQVLnbsS",
            "https://twitter.com/TheNowRockHD",
            true);
```
