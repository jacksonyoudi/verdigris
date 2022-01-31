use clap::{AppSettings, Clap};

// 定义HTTpie的cli入口，它包含若干个子命令
// 下面 ///  的注释是文档， clap会将其座位CLI的帮助


#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "youdi <liangchangyoujackson@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}


//  子命令分别对应不同的http方法，目前只支持get /post
#[derive(Clap, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

#[derive(Clap, Debug)]
struct Get {
    url: String,
}


#[derive(Clap, Debug)]
struct Post {
    url: String,
    body: Vec<String>,
}





fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);

}
