#[derive(Debug)]
struct Student {
    name: String,
    gpa: f32,
}

fn main() {
    let students = vec!["Riccardo 1.0", "Marco 2.0", "Tommaso 3.5", "Stefano 4.0"];

    // Cleaner approach
    let good_students_2: Vec<Student> = students
        .iter()
        .map(|s| {
            let mut s = s.split(' ');
            let name = s.next()?.to_owned();
            let gpa = s.next()?.parse::<f32>().ok()?;

            Some(Student { name, gpa })
        })
        .flatten() // gets ride of the None variants
        .filter(|s| s.gpa >= 3.5)
        .collect(); // turns iterator into a collection

    let mut good_students = vec![];
    for item in students {
        let mut s = item.split(' ');
        let name: Option<&str> = s.next();
        let gpa: Option<&str> = s.next();

        if let (Some(name), Some(gpa)) = (name, gpa) {
            let name = name.to_owned();
            let gpa = gpa.parse::<f32>();

            if let Ok(gpa) = gpa {
                if gpa >= 3.5 {
                    good_students.push(Student { name, gpa })
                }
            }
        }
    }

    for s in good_students {
        println!("{s:?}");
    }
}
