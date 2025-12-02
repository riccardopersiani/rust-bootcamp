// Note: Rust does not support classic inheritance. Rust support traits to achieve polymorphism.

// The only functionality related to parking is abstracted into a trait.
trait Park {
    fn park(&self);
}

trait Paint {
    fn paint(&self, color: String) {
        println!("Painting with color: {}", color);
    }
}

struct VehicleInfo {
    make: String,
    model: String,
    year: u16,
}

// Car

struct Car {
    info: VehicleInfo,
}

impl Park for Car {
    fn park(&self) {
        println!(
            "Parking the car: {} {} {}",
            self.info.year, self.info.make, self.info.model
        );
    }
}

impl Paint for Car {
    fn paint(&self, color: String) {
        println!(
            "Painting the car: {} {} {} with color {}",
            self.info.year, self.info.make, self.info.model, color
        );
    }
}

// Truck

struct Truck {
    info: VehicleInfo,
}

impl Truck {
    fn unload(&self) {
        println!(
            "Unloading the truck: {} {} {}",
            self.info.year, self.info.make, self.info.model
        );
    }
}

impl Park for Truck {
    fn park(&self) {
        println!(
            "Parking the truck: {} {} {}",
            self.info.year, self.info.make, self.info.model
        );
    }
}

impl Paint for Truck {
    fn paint(&self, color: String) {
        println!(
            "Painting the truck: {} {} {} with color {}",
            self.info.year, self.info.make, self.info.model, color
        );
    }
}

struct House {}

impl Paint for House {
    fn paint(&self, color: String) {
        println!("Painting the house with color {}", color);
    }
}

// Gets truck, car or house and paint it red.
fn paint_red<T>(object: &T) {
    // Error: no method paint was found as for reference. We don't know if the object implement that method.
    // object.paint("red".to_owned());
}

// Here we enforce the trait paint()
fn paint_red2<T: Paint>(object: &T) {
    // Error: no method paint was found as for reference. We don't know if the object implement that method.
    object.paint("red".to_owned());
}

// Here we enforce the trait paint() in another way
fn paint_red3(object: &impl Paint) {
    object.paint("red".to_owned());
}

// this is another way
fn paint_vehicle_red<T>(object: &T)
where
    // T must be any type that implements both traits
    T: Paint + Park,
{
    object.paint("red".to_owned());
}

fn create_paintable_object() -> impl Paint {
    House {}
}

fn main() {
    let car = Car {
        info: VehicleInfo {
            make: String::from("make"),
            model: String::from("model"),
            year: 2015,
        },
    };
    let house = House {};
    let object = create_paintable_object();

    // all pass cause all implement the pain method
    paint_red2(&car);
    paint_red2(&house);
    paint_red2(&object);

    //
    paint_vehicle_red(&car);
    // error cause they don't implement the park trait
    // paint_vehicle_red(&house);
    // paint_vehicle_red(&object);
}
