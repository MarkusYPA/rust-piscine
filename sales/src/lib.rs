#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Default::default(),
            receipt: Default::default(),
        }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        // could use match to avoid panic
        self.items.push(
            s.products
                .iter()
                .find(|(name, _)| *name == ele)
                .unwrap()
                .to_owned(),
        );
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        // get prices without item names
        // sort
        // len() / 3 is how many free: 9 > 3, 11 > 3
        // for each price, multiply by 1 - (free items price) / (all items price) and round to 2 decimal places

        let mut prices = self
            .items
            .iter()
            .map(|(_, price)| price.clone())
            .collect::<Vec<f32>>();
        prices.sort_by(|a, b| a.total_cmp(b));

        let frees_price = prices[..(prices.len() / 3)].iter().sum::<f32>();

        self.receipt = prices
            .iter()
            .map(|p| {
                (*p * (1.0 - frees_price / prices.iter().sum::<f32>()) * 100.0).round() / 100.0
            })
            .collect();

        self.receipt.to_owned()
    }
}

#[cfg(test)]
mod tests;
