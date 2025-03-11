use std::io::Error;

#[derive(Default, Debug)]
enum Person {
    Student {
        name: String,
        estimates: Vec<u32>,
    },
    Teacher {
        name: String,
        salary: u32,
    },
    #[default]
    Other,
}

#[derive(Default)]
struct PersonBuilder {
    person: Person,
}

impl PersonBuilder {
    fn new() -> PersonBuilder {
        PersonBuilder::default()
    }

    fn name(mut self, name_to_set: &str) -> Self {
        match self.person {
            Person::Student {
                ref mut name,
                estimates: _,
            } => {
                *name = name_to_set.to_string();
            }
            Person::Teacher {
                ref mut name,
                salary: _,
            } => {
                *name = name_to_set.to_string();
            }
            Person::Other => {}
        }

        self
    }
    fn estimates(mut self, estimates_to_set: Vec<u32>) -> Self {
        match self.person {
            Person::Student {
                name: _,
                ref mut estimates,
            } => {
                *estimates = estimates_to_set;
            }
            Person::Teacher { name: _, salary: _ } => {}
            Person::Other => {}
        }

        self
    }

    fn salary(mut self, salary_to_set: u32) -> Self {
        match self.person {
            Person::Teacher {
                name: _,
                ref mut salary,
            } => {
                *salary = salary_to_set;
            }
            Person::Student {
                name: _,
                estimates: _,
            } => {}
            Person::Other => {}
        }

        self
    }

    fn student(mut self) -> Self {
        self.person = Person::Student {
            name: "".to_string(),
            estimates: vec![],
        };
        self
    }

    fn teacher(mut self) -> Self {
        self.person = Person::Teacher {
            name: "".to_string(),
            salary: 0,
        };
        self
    }

    fn other(mut self) -> Self {
        self.person = Person::Other;
        self
    }

    fn build(self) -> Person {
        self.person
    }
}

trait SchoolRepository {
    async fn save_student(&mut self, student: &Person) -> Result<(), Error>;
    async fn save_teacher(&mut self, teacher: &Person) -> Result<(), Error>;
}

struct DbConnection;

impl DbConnection {
    async fn new() -> DbConnection {
        println!("Connecting to DB...");

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        println!("DB connected");
        DbConnection
    }
}

impl Drop for DbConnection {
    fn drop(&mut self) {
        println!("DB disconnected");
    }
}

struct DbSchoolRepository {
    connection: DbConnection,
}

impl DbSchoolRepository {
    async fn new() -> DbSchoolRepository {
        let connection = DbConnection::new().await;
        DbSchoolRepository { connection }
    }
}

impl SchoolRepository for DbSchoolRepository {
    async fn save_student(&mut self, student: &Person) -> Result<(), Error> {
        let _ = &self.connection;

        print!("Saving student...");
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        println!("Student {:?} saving done", student);
        Ok(())
    }

    async fn save_teacher(&mut self, teacher: &Person) -> Result<(), Error> {
        let _ = &self.connection;

        print!("Saving teacher...");
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        println!("Saving {:?} teacher done", teacher);
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let student = PersonBuilder::new()
        .other()
        .student()
        .name("John")
        .estimates(vec![1, 2, 3])
        .build();

    let teacher = PersonBuilder::new()
        .teacher()
        .name("John")
        .salary(100)
        .build();

    let mut repo = DbSchoolRepository::new().await;

    println!("Create new student and teacher");
    println!("{:?}", student);
    println!("{:?}", teacher);

    repo.save_student(&student).await.unwrap();
    repo.save_teacher(&teacher).await.unwrap();
}
