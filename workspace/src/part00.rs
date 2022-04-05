// Rust-101, Part 00: Algebraic datatypes
// ======================================

// As our first piece of Rust code, we want to write a function that computes the
// minimum of a list.

enum NothingOrNumber {
    Number(i32),
    Nothing
}

/*
    Давайте начнем с размышлений о типе нашей функции. Rust заставляет нас
    указывать типы всех аргументов и тип возвращаемого значения еще до того,
    как мы начнем писать тело. В случае с нашей минимальной функцией мы можем
    сказать, что она возвращает число. Но тогда у нас возникнут проблемы:
     - каков минимум пустого списка?
    Тип функции говорит, что мы должны что-то вернуть. Мы могли бы просто выбрать
    0, но это было бы произвольно. Нам нужен тип, который является «числом или ничем».
    Такой тип (с несколькими исключительными параметрами) называется
     - «алгебраическим типом данных», и Rust позволяет нам определять такие типы с
     помощью ключевого слова enum.
     Исходя из C(++), вы можете думать о таком типе как об объединении вместе с полем,
     в котором хранится вариант объединения, который используется в данный момент.
*/

/*
    Нам не нужно писать тип рядом с min, Rust может понять это автоматически
    (немного похоже на auto в C++11). Также обратите внимание на mut:
    в Rust переменные неизменяемы по умолчанию, и вам нужно сообщить Rust,
    если вы хотите изменить переменную позже. (keyword mut)

    Pattern matching:
    Обратите внимание, что Rust следит за тем, чтобы вы не забыли обработать ни один
    регистр в своем матче. Мы говорим, что сопоставление с образцом должно быть исчерпывающим.
*/
// Observe how in Rust, the return type comes *after* the arguments.

/*
    Действительно, мы можем: Следующая строка сообщает Rust, что конструкторы
    NumberOrNothing должны быть помещены в локальное пространство имен. Попробуйте
    переместить это над функцией и удалить все вхождения NumberOrNothing::
*/

use self::NothingOrNumber::{Nothing, Number};

fn vec_min(vec: Vec<i32>) -> NothingOrNumber {
    let mut min = Nothing;
    for el in vec {
        match min {
            Nothing => {
                min = Number(el);
            },
            Number(n) => {
                min = Number(min_i32(el, n));
            }
        }
    }
    return min;
}

fn min_i32(a: i32, b:i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}

/*
    vec! это макрос (обозначенный !), который создает константу Vec<_> с заданными элементами.
*/

fn read_vec() -> Vec<i32> {
    vec![5,4,-1,3,2]
}

fn print_nothing_or_number(n: NothingOrNumber) {
    match n {
        Nothing => println!("Nothing"),
        Number(n) => println!("Number={}", n),
    };
}

// Putting it all together:
pub fn main() {
    let vec = read_vec();
    let min = vec_min(vec);
    print_nothing_or_number(min);
}

// Finally, try `cargo run` on the console to run it.


