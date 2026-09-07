struct Fridge {
    brand: String,
    color: String,
    temperature: String,
    price: i32,
    has_ice_machine: bool,
    fridgey: bool,
    can_run_doom: bool,
}

fn main() {
    let fridge = Fridge {
        brand: String::from("samsung"),
        color: String::from("black"),
        temperature: String::from("very cold"),
        price: 69420,
        has_ice_machine: true,
        fridgey: true,
        can_run_doom: true,
    };
    let brand = &fridge.brand;
    let color = &fridge.color;
    let temperature = &fridge.temperature;
    let price = fridge.price;
    let ice = fridge.has_ice_machine;
    let is_fridgey = fridge.fridgey;

    println!("brand: {brand}");
    println!("color: {color}");
    println!("temperature: {temperature}");
    println!("price: ${price}");
    println!("ice machine: {ice}");
    println!("is the fridge fridgey? {is_fridgey}");

    if fridge.can_run_doom {
    	println!("can it?? yes it does!!");
    } else {
    	println!("awww..");
    }
}

