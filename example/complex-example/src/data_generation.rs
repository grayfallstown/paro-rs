use uuid::Uuid;
use rand::Rng;
use std::sync::Arc;
use crate::state::*;


pub fn generate_employees(count: usize) -> Vec<Arc<Employee>> {
    let first_names = vec![
        "John", "Jane", "Mike", "Emily", "Jessica", "Jacob", "Emily", 
        "Michael", "Matthew", "Amanda", "Daniel", "Joshua", "Andrew", 
        "David", "Brian", "James", "Robert", "Megan", "Ashley",
        "Mohammed", "Fatima", "Ahmed", "Maria", "Sophia", "Liam", "Emma", 
        "Noah", "Olivia", "Ava", "Isabella", "Mia", "Charlotte", "Amelia",
        "Emily", "Abigail", "Madison", "Elizabeth", "Sofia", "Avery",
        "Evelyn", "Hannah", "Aubree", "Addison", "Aaliyah", "Riley",
        "Harper", "Aria", "Aryan", "Arianna", "Camila", "Kaylee", "Aurora",
        "Ruby", "Lauren", "Bella", "Natalie", "Zoe", "Leah", "Hazel",
        "Violet", "Aurora", "Savannah", "Audrey", "Brooklyn", "Brielle",
        "Karina", "Makayla", "Maria", "Makayla", "Allison", "Nevaeh",
        "Avery", "Makayla", "Brooklyn", "Bella", "Aubree", "Brooklyn",
        "Brielle", "Aurora", "Aaliyah", "Rylee", "Avery", "Aurora", "Aubree"
    ];

    let last_names = vec![
        "Smith", "Johnson", "Williams", "Jones", "Brown", "Davis", "Miller", 
        "Wilson", "Moore", "Taylor", "Anderson", "Thomas", "Jackson", "White", 
        "Harris", "Martin", "Thompson", "Garcia", "Martinez", "Robinson", "Clark",
        "Rodriguez", "Lewis", "Lee", "Walker", "Hall", "Allen", "Young",
        "Hernandez", "King", "Wright", "Lopez", "Hill", "Scott", "Green",
        "Adams", "Baker", "Gonzalez", "Nelson", "Carter", "Mitchell", "Perez",
        "Roberts", "Turner", "Phillips", "Campbell", "Parker", "Evans", "Edwards",
        "Stewart", "Flores", "Morris", "Nguyen", "Murphy", "Rivera", "Cook",
        "Rogers", "Morgan", "Peterson", "Cooper", "Reed", "Bailey", "Bell",
        "Gomez", "Kelly", "Howard", "Ward", "Cox", "Diaz", "Richardson",
        "Wood", "Watson", "Brooks", "Bennett", "Gray", "James", "Reyes",
        "Cruz", "Hughes", "Price", "Myers", "Long", "Foster", "Sanders",
        "Ross", "Morales", "Powell", "Sullivan", "Russell",
    ];
    let departments = vec![
        Department::Sales,
        Department::Sales,
        Department::Sales,
        Department::Sales,
        Department::Sales,
        Department::Production,
        Department::Production,
        Department::Production,
        Department::Production,
        Department::Production,
        Department::Production,
        Department::Production,
        Department::Production,
        Department::Production,
        Department::Production,
        Department::Production,
        Department::Management,
        Department::Maintenance,
        Department::Maintenance,
        Department::Maintenance,
        Department::Maintenance,
    ];
    let mut rng = rand::thread_rng();
    

    let mut employees = vec![];
    for i in 0..count {
        let id = Uuid::new_v4().to_string();
        let first_name_dist = rand::distributions::Uniform::new(0, first_names.len());
        let last_name_dist = rand::distributions::Uniform::new(0, first_names.len());
        let departments_dist = rand::distributions::Uniform::new(0, departments.len());
        
        let first_name = first_names[rng.sample(first_name_dist)];
        let last_name = last_names[rng.sample(last_name_dist)];
        let login = format!("{}.{}-{}", first_name, last_name, i);
        let department = departments[rng.sample(departments_dist)];
        let employee = Employee {
            id,
            first_name: first_name.to_owned(),
            last_name: last_name.to_owned(),
            login,
            department,
        };
        employees.push(Arc::new(employee));
    }

    employees
}
