fn main(){
    let dog = Dog{};
    let cat = Cat{};
    show_animal_data(dog);
    show_animal_data(cat);
}

trait Animal {
    // 動物の寿命を返す
    fn lifespan(&self) -> u32;
    // 動物の学術名を返す
    fn scientific_name(&self) -> String;
}

// 犬の構造体
struct Dog;

// 犬の構造体に対する lifespan()関数と scientific_name()関数の定義
impl Animal for Dog{
    fn lifespan(&self) -> u32{
        13
    }
    fn scientific_name(&self) -> String{
        "Canis lupus familiaris".to_string()
    }
}


//猫の構造体を用意する
struct Cat;

impl Animal for Cat{
    fn lifespan(&self) -> u32{
        16
    }
    fn scientific_name(&self)  -> String{
        "Felis catus".to_string()
    }
}


fn show_animal_data<T: Animal>(animal: T){
    println!("Lifespan: {} years", animal.lifespan());
    println!("Scientific name: {}", animal.scientific_name());
}
