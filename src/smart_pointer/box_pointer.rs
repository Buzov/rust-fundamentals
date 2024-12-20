trait Vehicle {
    fn drive(&self);
}

struct Truck;
// next_truck: Option<Box<Truck>>,

impl Vehicle for Truck {
    fn drive(&self) {
        println!("Truck is driving")
    }
}

pub fn run_truck() -> () {
    let t: Box<dyn Vehicle>;
    t = Box::new(Truck);
    t.drive();
}