fn main() {
    let data = vec![1, 2, 3, 4];
    let d1 = &data;
    println!("address of value: {:p}({:p}), address of data: {:p}, address of data1: {:p}", &data, d1, &&data, &d1);
    println!("sum of d1: {}", sum(d1));
    println!("address of items: [{:p}, {:p}, {:p}, {:p}]", &data[0], &data[1], &data[2], &data[3]);

    let mut a = Account::new("oldsix".to_string(), 26, 171, 60);

    println!("account {:?}", a);

    println!("bmi: {}", a.bmi());

    a.tall(11);
    &a.tall(12);

    println!("account {:?}", a);

    println!("bmi: {}", a.bmi());
    println!("bmi: {}", &a.bmi());
    println!("bmi: {}", &&a.bmi());

    println!("name: {}", a.name());
    // a.name();
}

#[derive(Debug)]
struct Account {
    name: String,
    age: u32,
    height: u32,
    weight: u32,
}

impl Account {

    fn new(name: String, age: u32, height: u32, weight: u32) -> Self {
        Self {
            name,
            age,
            height,
            weight,
        }
    }

    fn tall(&mut self, length: u32) {
        self.height += length
    }

    fn bmi(&self) -> f32 {
        let height = self.height as f32 / 100_f32;
        let square = height * height;
        self.weight as f32 / square
    }

    fn modify_name(mut self) -> String {
        self.name.push_str("_haha");
        self.name
    }

    fn name(self) -> String {
        self.name
    }
}

fn sum(data: &Vec<u32>) -> u32 {
    println!("address of value: {:p}, address of ref: {:p}", data, &data);
    data.iter().fold(0, |acc, x| acc + x)
}