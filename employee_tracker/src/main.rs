use std::collections::HashMap;

fn main() {
    println!("-----| EMPLOYEE TRACKER |-----");
    let mut dm = DepartmentMap::new();

    dm.add_to_department("engineering", "Alex Henderson");
    dm.add_to_department("engineering", "Carl Pattie");
    dm.add_to_department("engineering", "Matt Chapman");

    dm.add_to_department("product", "Leonie Wise");

    dm.list_all_departments();
}

#[derive(Debug)]
struct DepartmentMap {
    map: HashMap<String, Vec<String>>,
}

impl DepartmentMap {
    fn new() -> DepartmentMap {
        DepartmentMap {
            map: HashMap::new(),
        }
    }

    fn add_to_department(&mut self, department_name: &str, employee: &str) {
        let department = self.map.get_mut(department_name);

        match department {
            Some(department) => department.push(String::from(employee)),
            None => {
                self.map
                    .insert(String::from(department_name), vec![String::from(employee)]);
            }
        }
    }

    fn list_all_departments(&self) {
        for (key, value) in &self.map {
            println!("{}", key.to_uppercase());
            println!("\t{}", value.join(", "));
        }
    }
}
