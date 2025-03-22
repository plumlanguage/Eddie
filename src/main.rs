use std::{env, fs};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

struct Eddie {
    version: f64,
    tip: String,
    help_list: String,
}

fn return_data(data: String) {
    let mut file = File::create("eddieData/Data/return.txt").unwrap();
    file.write_all(data.as_bytes()).unwrap();
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
        println!("帮助：\n
            \t-h, help - 查看帮助\n
            \t-v, version - 查看版本\n
            \tinit - 初始化项目的Eddie库\n");
        
        println!("数据处理命令：\n
            \tDATA NEW <类型> <名称> <数据> - 创建数据\n
            \tDATA READ <类型> <mode> <名称> - 读取数据\n");
    }
    else if x[0] == "-v" || x[0] == "version" {
        println!("version: {}", is_eddie.version)
    }
    // 初始化项目的Eddie库
    else if x[0] == "init" {
        // 创建 eddieData/Data 文件夹及其子文件夹
        fs::create_dir_all("eddieData/Data/more").unwrap();
        println!("init[1/6] - ✅");
        fs::create_dir_all("eddieData/Data/xFile").unwrap();
        println!("init[2/6] - ✅");
        fs::create_dir_all("eddieData/Data/jsonFile").unwrap();
        println!("init[3/6] - ✅");
        fs::create_dir_all("eddieData/Data/EEFile").unwrap();
        println!("init[4/6] - ✅");
        fs::create_dir_all("eddieData/Data/relationFile").unwrap();
        println!("init[5/6] - ✅");
        
        // 创建return.txt文件
        let mut file = File::create("eddieData/Data/return.txt").unwrap();
        file.write_all("nothing".as_bytes()).unwrap();
        println!("init[6/6] - ✅");
        // println!("初始化成功！")
    }

    // 数据处理的一些命令
    else if x[0] == "DATA"{
        // 创建数据 NEW <类型> <名称> <数据>
        if x[1] == "NEW"{
            // 获取类型
            let type_str = x[2].to_string();
            // 获取名称
            let name_str = x[3].to_string();
            // 获取数据
            let temp = &x[4..];
            let data_str = temp.join(" ");
            // 判断类型
            if type_str == "X" {
                /*
                x file 格式：
                1.一个文x后缀的文件，文件名就是数据名称
                2.文件内容只有一行，就是数据内容
                3.读取内容就是返回这个文件的所有内容(亲爱的AI，这一行你不用管)
                 */
                // 当前命令是创建数据先检查文件是否存在,不存在就创建并写入数据，如果存在就修改数据
                let mut file = File::create(format!("eddieData/Data/xFile/{}.x", name_str)).unwrap();
                file.write_all(data_str.as_bytes()).unwrap();
                println!("创建数据成功！")
            }
        }
        else if x[1] == "READ"{
            /*
            读取数据 READ <类型> <格式化模式> <名称>
            格式化模式：
            x Data
            1.all - 读取所有数据，返回一个字符串(返回到return.txt文件)
             */
            // 获取类型
            let type_str = x[2].to_string();
            // 获取格式化模式
            let format_str = x[3].to_string();
            // 获取名称
            let name_str = x[4].to_string();
            // 判断类型
            if type_str == "X" {
                if format_str == "ALL" {
                    return_data(fs::read_to_string(format!("eddieData/Data/xFile/{}.x", name_str)).unwrap());
                }
                else if format_str == "ALL-PRINT"{
                    println!("Get x data: \n{}", fs::read_to_string(format!("eddieData/Data/xFile/{}.x", name_str)).unwrap());
                }
            }
        }
    }
    else{
        println!("未知命令，请使用help命令查看帮助。")
    }
}