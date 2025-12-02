// Note: Rust does not support classic inheritance. Rust support traits to achieve polymorphism.

// The only functionality related to parking is abstracted into a trait.
trait Vehicle: Paint {
    fn park(&self);
    fn get_default_color() -> String {
        "black".to_owned()
    }
}

// Supertrait
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

impl Vehicle for Car {
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

// Here we enforce to accept a trait object, now accepts a reference to some type that implements the Paint trait
fn paint_red2(object: &dyn Paint) {
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
    T: Vehicle,
{
    object.paint("red".to_owned());
}

// Box points to something allocated to the heap
// static dispatch: knows what to call at compile time
// generics are substituted to concrete objects at compile time
//
// dynamic dispatch - compiler cannot figure which concrete method to call, so it needs to be done at run time
// the advantage is that is more flexible, disadvantage is performance
fn create_paintable_object(vehicle: bool) -> Box<dyn Paint> {
    if vehicle {
        Box::new(Car {
            info: VehicleInfo {
                make: "Honda".to_owned(),
                model: "Civic".to_owned(),
                year: 2000,
            },
        })
    } else {
        Box::new(House {})
    }
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
    let object = create_paintable_object(true);

    // ref to types that implement the Paint trait
    let paintable_objects: Vec<&dyn Paint> = vec![&car, &house];

    // all pass cause all implement the pain method
    paint_red2(&car);
    paint_red2(&house);
    // convert object from a box pointer to a reference
    paint_red2(object.as_ref());

    //
    paint_vehicle_red(&car);
    // error cause they don't implement the park trait
    // paint_vehicle_red(&house);
    // paint_vehicle_red(&object);
}

// trait objects are used when the compiler cannot tell what to use at compile type
