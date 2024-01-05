pub mod closures {
    use std::{thread, time::Duration};

    // anonymous functions like arrow functions in js, but here we dont have arrows, instead we
    // have pipes

    struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    pub fn generate_workout(intensity: u32, random_numer: u32) {
        let mut cached_result = Cacher::new(|num: u32| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        });

        if intensity < 25 {
            println!("Today, I did {} situps", cached_result.value(intensity));
        } else {
            println!("Today, ran for {} minutes!", cached_result.value(intensity));
        }
    }

    fn closure_taking_ownership_using_move_keyword() {
        let x = vec![1, 2, 3];
        let equal_to_x = move |z: Vec<i32>| z == x;
        //println!("can't use x here: {:?}",x); value moved inside closure
    }

    pub fn main() {
        let i = 3;
        let rn = 23;
        generate_workout(i, rn);
    }
}
