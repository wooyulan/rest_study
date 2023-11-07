

mod complex;
mod generics;
mod matches;
mod method;
mod process_control;
mod formats;
mod threads;
mod files;

fn main() {

    // 复合类型- 切片
    //complex::str_slice::test_str_slice();

    // 复合类型 - 元组
    // complex::tuple::test_tuple();

    // 复合类型 - 结构体
    //complex::t_struct::test_struct();

    // 复合类型 - 枚举
    // complex::t_enum::test_enum();

    // 复合类型 - 数组
    // complex::t_array::test_array();

    /*:::: 流程控制 ::: */
    // process_control::t_pro_ctrl::test_process_control();

    // 模型匹配
    // matches::t_match::test_match();

    // 方法
    //method::method::test_method();

    // 泛型特征
    // generics::gennerics::test_gennerics();
    // generics::traits::test_trait();

    // 格式化输出
    // formats::format::test_format();

    // 多线程
    // threads::t_thread::test_thread();

    // 入门实战 -- 文件操作
    files::file_option::test_file();






}
