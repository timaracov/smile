mod sys_info;

fn main() {
    println!("{}\n{}", 
             sys_info::get_user_info(), 
             sys_info::get_host_info());
}

