struct UniswapSimulator {
    reserve_a: f64,
    reserve_b: f64,
    constant_product: f64,
}

impl UniswapSimulator {
    fn new(reserve_a: f64, reserve_b: f64) -> Self {
        let constant_product = reserve_a * reserve_b;
        UniswapSimulator {
            reserve_a,
            reserve_b,
            constant_product,
        }
    }

    fn get_price(&self) -> f64 {
        self.reserve_b / self.reserve_a
    }

    fn swap(&mut self, amount_in: f64, token_in: &str) -> f64 {
        match token_in {
            "A" => {
                let new_reserve_a = self.reserve_a + amount_in;
                let new_reserve_b = self.constant_product / new_reserve_a;
                let amount_out = self.reserve_b - new_reserve_b;
                self.reserve_a = new_reserve_a;
                self.reserve_b = new_reserve_b;
                amount_out
            }
            "B" => {
                let new_reserve_b = self.reserve_b + amount_in;
                let new_reserve_a = self.constant_product / new_reserve_b;
                let amount_out = self.reserve_a - new_reserve_a;
                self.reserve_a = new_reserve_a;
                self.reserve_b = new_reserve_b;
                amount_out
            }
            _ => panic!("Invalid token type. Use 'A' or 'B'."),
        }
    }

    fn add_liquidity(&mut self, amount_a: f64, amount_b: f64) {
        self.reserve_a += amount_a;
        self.reserve_b += amount_b;
        self.constant_product = self.reserve_a * self.reserve_b;
    }

    fn remove_liquidity(&mut self, liquidity_fraction: f64) -> (f64, f64) {
        let amount_a = self.reserve_a * liquidity_fraction;
        let amount_b = self.reserve_b * liquidity_fraction;
        self.reserve_a -= amount_a;
        self.reserve_b -= amount_b;
        self.constant_product = self.reserve_a * self.reserve_b;
        (amount_a, amount_b)
    }

    fn change_constant_product(&mut self, value: f64) {
        self.constant_product += value;
    }
}

fn pool_swap_a() {
    let mut pool = UniswapSimulator::new(5000.0, 1000.0);

    println!("Initial price of A in terms of B: {:.2}", pool.get_price());

    let amount_b_received = pool.swap(10.0, "A");
    println!("Swapped 10 A for {:.2} B", amount_b_received);

    pool.add_liquidity(500.0, 500.0);
    println!("Added 500 A and 500 B to the pool.");

    let (amount_a_removed, amount_b_removed) = pool.remove_liquidity(0.5);
    println!(
        "Removed {:.2} A and {:.2} B from the pool.",
        amount_a_removed, amount_b_removed
    );
}

fn pool_plus_constant_product_swap_a() {
    let mut pool = UniswapSimulator::new(5000.0, 1000.0);

    pool.change_constant_product(50.0);

    println!("Initial price of A in terms of B: {:.2}", pool.get_price());

    let amount_b_received = pool.swap(10.0, "A");
    println!("Swapped 10 A for {:.2} B", amount_b_received);

    pool.add_liquidity(500.0, 500.0);
    println!("Added 500 A and 500 B to the pool.");

    let (amount_a_removed, amount_b_removed) = pool.remove_liquidity(0.5);
    println!(
        "Removed {:.2} A and {:.2} B from the pool.",
        amount_a_removed, amount_b_removed
    );
}

fn pool_minus_constant_product_swap_a() {
    let mut pool = UniswapSimulator::new(5000.0, 1000.0);

    pool.change_constant_product(-50.0);

    println!("Initial price of A in terms of B: {:.2}", pool.get_price());

    let amount_b_received = pool.swap(10.0, "A");
    println!("Swapped 10 A for {:.2} B", amount_b_received);

    pool.add_liquidity(500.0, 500.0);
    println!("Added 500 A and 500 B to the pool.");

    let (amount_a_removed, amount_b_removed) = pool.remove_liquidity(0.5);
    println!(
        "Removed {:.2} A and {:.2} B from the pool.",
        amount_a_removed, amount_b_removed
    );
}

fn pool_plus_constant_product_swap_b() {
    let mut pool = UniswapSimulator::new(5000.0, 1000.0);

    pool.change_constant_product(50.0);

    println!("Initial price of A in terms of B: {:.2}", pool.get_price());

    let amount_a_received = pool.swap(10.0, "B");
    println!("Swapped 10 B for {:.2} A", amount_a_received);

    pool.add_liquidity(500.0, 500.0);
    println!("Added 500 A and 500 B to the pool.");

    let (amount_a_removed, amount_b_removed) = pool.remove_liquidity(0.5);
    println!(
        "Removed {:.2} A and {:.2} B from the pool.",
        amount_a_removed, amount_b_removed
    );
}

fn pool_minus_constant_product_swap_b() {
    let mut pool = UniswapSimulator::new(5000.0, 1000.0);

    pool.change_constant_product(-50.0);

    println!("Initial price of A in terms of B: {:.2}", pool.get_price());

    let amount_a_received = pool.swap(10.0, "B");
    println!("Swapped 10 B for {:.2} A", amount_a_received);

    pool.add_liquidity(500.0, 500.0);
    println!("Added 500 A and 500 B to the pool.");

    let (amount_a_removed, amount_b_removed) = pool.remove_liquidity(0.5);
    println!(
        "Removed {:.2} A and {:.2} B from the pool.",
        amount_a_removed, amount_b_removed
    );
}

fn main() {
    pool_swap_a();
    println!("\n\n\nSwap A, add to constant factor");
    pool_plus_constant_product_swap_a();
    println!("\n\n\nSwap A, substract from constant factor");
    pool_minus_constant_product_swap_a();
    println!("\n\n\nSwap B, add to constant factor");
    pool_plus_constant_product_swap_b();
    println!("\n\n\nSwap B, substract from constant factor");
    pool_minus_constant_product_swap_b();

    println!(r#"
        If B is bad token:

        If swap A {{
            Substract from constant product
        }}

        If swap B {{
            Add to constant product
        }}
    "#);

    pool_minus_constant_product_swap_a();
    pool_plus_constant_product_swap_b();


}
