// Rust-101, Part 01: Expressions, Inherent methods
// ================================================

// ## Expression-based programming
/*
    Несмотря на то, что наш код из первой части работает, мы можем многому научиться,
    сделав его красивее. Это потому, что Rust — это язык, основанный на выражениях, а
    это означает, что большинство терминов, которые вы записываете, — это не просто
    операторы (выполнение кода), а выражения (возвращающие значение). Это относится даже
    к телу целых функций!
*/

/*
    Между фигурными скобками мы даем выражение, которое вычисляет возвращаемое значение.
    Так что мы можем просто написать i * i, выражение, которое возвращает квадрат i!
    Это очень похоже на то, как математики записывают функции (но с большим количеством типов).
*/
fn sqr(i: i32) -> i32 { i * i }

/*
    Условные выражения также являются просто выражениями.
    Это сравнимо с тройным ? : оператор из таких языков, как C.
*/
fn abs(i: i32) -> i32 { if i >= 0 { i } else { -i } }

enum NothingOrNumber{
    Nothing,
    Number(i32)
}

use self::NothingOrNumber::{Nothing, Number};

/*
    И то же самое относится к macth:
    каждое плечо совпадения дает выражение, которое возвращается в
    соответствующем случае. (Здесь мы повторяем определение из предыдущей части.)
*/

fn number_or_default(n: NothingOrNumber, default: i32) -> i32 {
    match n {
        Nothing => default,
        Number(n) => n,
    }
}

// Это даже тот случай, когда блоки являются выражениями, оценивающими
// последнее выражение, которое они содержат.
fn compute_stuff(x: i32) -> i32 {
    let y = { let z = x*x; z + 14 };
    y*y
}

// Рефакторинг vec_min
/*
    Помните вспомогательную функцию min_i32? Rust позволяет нам определять такие
    вспомогательные функции внутри других функций. Это просто вопрос пространства
    имен, внутренняя функция не имеет доступа к данным внешней. Тем не менее,
    возможность красиво сгруппировать функции может значительно улучшить читабельность.
*/
fn vec_min(v: Vec<i32>) -> NothingOrNumber {
    fn min_i32(a: i32, b: i32) -> i32 { if a <= b { a } else { b } }

    let mut min = Nothing;
    for e in v {
        min = Number(match min {
            Nothing => e,
            Number(n) => min_i32(e, n),
        });
    }
    min
}


// **Exercise 01.1**: Write a function `vec_sum` that computes the sum of all values of a `Vec<i32>`.
fn vec_sum(v: Vec<i32>) -> NothingOrNumber {
    let mut sum = Nothing;
    for e in v {
        sum = Number(match sum {
            Nothing => e,
            Number(n) => n + e,
        })
    }
    sum
}

// **Exercise 01.2**: Write a function `vec_print` that takes a vector and prints all its elements.
fn vec_print(v: Vec<i32>) {
    match v.len() {
        0 => print!("vec is empty"),
        n => {
            for e in v {
                print!("{} ", e);
            }
        }
    };
    println!();
}

// ## Inherent implementations
/*
    Вот вам и vec_min. Давайте теперь еще раз рассмотрим print_number_or_nothing.
    Эта функция действительно очень близка к типу NumberOrNothing. В C++ или Java вы,
    вероятно, сделали бы это методом типа. В Rust мы можем добиться чего-то очень похожего,
    предоставив встроенную реализацию.
*/

/*
    Rust отделяет код от данных, поэтому определение методов в перечислении
    (а также в структуре, о которой мы узнаем позже) не зависит от определения типа.
    self подобен этому в других языках, и его тип всегда подразумевается.
    Таким образом, print теперь является методом, который принимает в качестве первого
    аргумента NumberOrNothing, точно так же, как print_number_or_nothing.
*/
impl NothingOrNumber {
    fn print(self) {
        match self {
            Nothing => println!("Nothing"),
            Number(n) => println!("Number={}", n),
        };
    }

    fn default(self, default: i32) -> i32 {
        match self {
            Nothing => default,
            Number(n) => n,
        }
    }
}

// With our refactored functions and methods, `main` now looks as follows:
fn read_vec() -> Vec<i32> {
    vec![18,5,7,2,9,27]
}
pub fn main() {
    let vec1 = read_vec();
    let min = vec_min(vec1);
    min.print();
    let vec2 = read_vec();
    vec_print(vec2);
    let vec3 = read_vec();
    let sum = vec_sum(vec3);
    sum.print();
}

