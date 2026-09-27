struct Fridge {
    brand: String,
    color: String,
    temperature: i32,
    price: i32,
    has_ice_machine: bool,
    fridgey: bool,
    can_run_doom: bool,
}

fn main() {
    let fridge = Fridge {
        brand: String::from("samsung"),
        color: String::from("black"),
        temperature: 69,
        price: 420,
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
        println!("yaey");
    } 
    else {
        println!("aw. why though?");
    }
    
match fridge.temperature {
    ..69 => println!("no more nice.. what have you done?"),
    69 => println!("yippee! u have satisfied the gods of the internet culture"),
    70.. => println!("can u at least make it 420 celsius? oh wait thats gonna melt ur house scratch that"),
}
}
