/**
 * 所有权练习
 * 修复代码的错误，让编译和运行通过，题目链接
 * https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=8f7672bce51464629ceaba5bb20b771c
 */

//fix. 支持 assert_eq
//#[derive(Debug)]
#[derive(Debug, PartialEq)]
struct Employee {
    name: String,
    age: u8,
    title: String,
}

impl Employee {
    pub fn new(name: String, age: u8, title: String) -> Self {
        Self { name, age, title }
    }
}

fn main() {
    let tyr = Employee::new("Tyr".to_owned(), 18, "CFO".to_string());
    let lindsey = Employee::new("Lindsey".to_owned(), 8, "CEO".to_string());
    let rosie = Employee::new("Rosie".to_owned(), 4, "CPO".to_string());
    // fix. fix_age 会修改 employees
    // let employees = vec![tyr, lindsey, rosie];
    let mut employees = vec![tyr, lindsey, rosie];

    // fix. tyr 已经 move
    //println!("tyr: {:?}", tyr);
    println!("tyr: {:?}", employees[0]);

    // fix. fix_age 会修改 employees
    fix_age(&mut employees);

    // 我们希望在 fix_age 后还能使用 employees
    println!("employees: {:?}", employees);
    assert_eq!(
        employees,
        &[
            Employee::new("Tyr".to_owned(), 18, "CFO".to_owned()),
            Employee::new("Lindsey".to_owned(), 18, "CEO".to_owned()),
            Employee::new("Rosie".to_owned(), 14, "CPO".to_owned())
        ]
    )
}

// fix. fix_age 会修改 employees
//fn fix_age(employees: Vec<Employee>) {
fn fix_age(employees: &mut Vec<Employee>) {
    employees
        //.iter()
        .iter_mut()
        //.filter(|e| e.age < 8)
        .filter(|e| e.age <= 8)
        .for_each(|e| e.age += 10);
}
