use std::env;
use std::fs::File;
use std::io::Write;

struct Eddie {
    version: f64,
    tip: String,
    help_list: String,
}
fn main() {
    // 设置当前版本数据
    let is_eddie: Eddie = Eddie{version: 0.1f64, tip: "第一个版本，完成一些基础的铺垫功能。".to_owned(), help_list: "👌".to_string() };
    
    
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    // 处理数据
    let x: Vec<String> = args[1..].to_owned();

    // 判断数据
    if x[0] == "-h" || x[0] == "help" {
        println!("帮助：\n\t-h, help - 查看帮助\n\t-v, version - 查看版本\n\tinit - 初始化项目的Eddie库")
    }
    else if x[0] == "-v" || x[0] == "version" {
        println!("version: {}", is_eddie.version)
    }
    // 初始化项目的Eddie库
    else if x[0] == "init" {
        //在当前目录下创建eddieData文件夹
        let mut file = File::create("eddieData").unwrap();
        file.write_all(b"Hello, world!").unwrap();
        println!("init[1] - ✅");
        //在eddieData文件夹下创建Data文件夹
        let mut file = File::create("eddieData/Data").unwrap();
        file.write_all(b"Hello, world!").unwrap();
        println!("init[2] - ✅");
        //在eddieData\\Data文件夹下创建more,xFile,jsonFile,EEFile,relationFile文件夹
        let mut file = File::create("eddieData/Data/more").unwrap();
        file.write_all(b"Hello, world!").unwrap();
        println!("init[3] - ✅");
        let mut file = File::create("eddieData/Data/xFile").unwrap();
        file.write_all(b"Hello, world!").unwrap();
        println!("init[4] - ✅");
        let mut file = File::create("eddieData/Data/jsonFile").unwrap();
        file.write_all(b"Hello, world!").unwrap();
        println!("init[5] - ✅");
        let mut file = File::create("eddieData/Data/EEFile").unwrap();
        file.write_all(b"Hello, world!").unwrap();
        println!("init[6] - ✅");
        let mut file = File::create("eddieData/Data/relationFile").unwrap();
        file.write_all(b"Hello, world!").unwrap();
        println!("init[7] - ✅");
    }

    // 数据处理的一些命令
    else if x[0] == "data"{
        // 创建数据 NEW <类型> <名称> <数据>
        if x[0] == "NEW"{
            // 获取类型
            let type_str = x[1].to_string();
            // 获取名称
            let name_str = x[2].to_string();
            // 获取数据
            let data_str = x[3].to_string();
        }
    }
    else{
        println!("未知命令，请使用help命令查看帮助。")
    }
}