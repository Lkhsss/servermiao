use actix_files as fs;
use actix_web::{App, HttpServer};
use std::{env, path::Path};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //从命令行中获取参数
    let args: Vec<String> = env::args().collect();
    let mut port = 8000;
    let mut path = String::from(".");
    for arg in args.into_iter().skip(1) {
        if if_Path(&arg) {
            path = arg.parse().unwrap()
        } else if if_port(&arg) {
            port = arg.parse().unwrap()
        }
    }
    let serverpath = Path::new(&path).to_owned().canonicalize().unwrap();
    println!(
        "[INFO] 在[ 0.0.0.0:{} ]上启动服务\n目标文件夹：[{}]",
        port,
        serverpath.display()
    );
    HttpServer::new(move || {
        App::new().service(
            fs::Files::new("/", &serverpath)
                .show_files_listing() //是否自动打开index.html
                .index_file("index.html")
                .use_etag(true) //是否返回etag
                .use_hidden_files() //是否使用隐藏文件
                .prefer_utf8(true) //是否优先使用utf8编码
                .use_last_modified(true),
        )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

fn if_port(num: &str) -> bool {
    match num.parse::<u16>() {
        Ok(_) => true,
        Err(_) => false,
    }
}
fn if_Path(path: &str) -> bool {
    let p = Path::new(path);
    if p.is_dir() {
        true
    } else {
        false
    }
}
