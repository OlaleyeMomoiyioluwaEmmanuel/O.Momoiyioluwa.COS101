struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    // Method to calculate cost for a given quantity
    fn cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    // Define laptops with their prices
    let hp = Laptop { brand: "HP".to_string(), price: 650_000 };
    let ibm = Laptop { brand: "IBM".to_string(), price: 755_000 };
    let toshiba = Laptop { brand: "Toshiba".to_string(), price: 550_000 };
    let dell = Laptop { brand: "Dell".to_string(), price: 850_000 };

    // Customer buys 3 from each brand
    let quantity = 3;

    // Calculate total cost
    let total_cost = hp.cost(quantity)
        + ibm.cost(quantity)
        + toshiba.cost(quantity)
        + dell.cost(quantity);

    println!("Total cost for 3 laptops from each brand = ₦{}", total_cost);
}
