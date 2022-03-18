// Rust-101, Part 05: Clone
// ========================

// ## Big Numbers

// будем использовать little endian порядок (сначала идут младшие значащие числа)
// также чтобы одно и тоже ичсло можно было сохранить только одним способом
// уберем завершающие нули.
// n = a0*l^0 + a1*l^1 + ... + ak*l^k => a0 - младшее число, ak - старшее
// a0a1...ak - little endian
// ak...a1a0 - big endian

// Заводим структуру. Поля структуры мб приватными для текущего модуля (по дефолту)
// или общедоступными (pub перед объявлением). В реальной программе лучше
// оставить приватным поле, чтобы гарантировать сохранение инварианта 
// нет заверщаюих нулей
pub struct BigInt {
    pub data: Vec<u64>, // least significant digit first, no trailing zeros
}

// Now that we fixed the data representation, we can start implementing methods on it.
impl BigInt {
    // конструктор, создающий BigInt из обычного целого числа.
    pub fn new(x: u64) -> Self {
        if x == 0 {
            BigInt{ data: vec![]}
        } else {
            BigInt{ data: vec![x]}
        }
    }

    // проверка структуры на инвариант
    pub fn test_invariant(&self) -> bool {
        if self.data.len() == 0 {
            true
        } else {
            self.data[self.data.len() - 1] != 0
        }
    }

    // We can convert any little-endian vector of digits (i.e., least-significant digit first) into
    // a number, by removing trailing zeros. The `mut` declaration for `v` here is just like the
    // one in `let mut ...`: We completely own `v`, but Rust still asks us to make our intention of
    // modifying it explicit. This `mut` is *not* part of the type of `from_vec` - the caller has
    // to give up ownership of `v` anyway, so they don't care anymore what you do to it.
    // 
    // **Exercise 05.1**: Implement this function.
    // 
    // *Hint*: You can use `pop` to remove the last element of a vector.
    pub fn from_vec(mut v: Vec<u64>) -> Self {
        let mut new_big = BigInt { data: vec![]};
        while v.is_empty() != true {
            match v.pop() {
                None => (),
                Some(n) => new_big.data.push(n),
            };
        }

        new_big
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn max_digit(&self) -> Option<u64> {
        use std::cmp;

        let mut max = None;
        for e in self.data.iter() {
            max = Some(match max{
                None => *e,
                Some(n) => cmp::max(n, *e)
            });
        }
        max
    }

    pub fn min_digit(&self) -> Option<u64> {
        use std::cmp;

        let mut min = None;
        for e in self.data.iter() {
            min = Some(match min {
                None => *e,
                Some(n) => cmp::min(n, *e)
            });
        }
        min
    }
}

// ## Cloning
// from_vec завладевает вектором, но чтобы всеравно не терять доступ к нему можно
// явно сделать полную копию. Технически клон берет вектор в виде shared ссылки
// и возвращает полностью новый. Но для копирования тип должен поддерживать
// трейт Clone.
fn clone_demo() {
    let v = vec![0,1 << 16];
    let b1 = BigInt::from_vec((&v).clone());
    let b2 = BigInt::from_vec(v);
}

// Сделать тип клонируемы это очень распространенная операция и Раст может сам это сделать.
// нужно только добавить #[derive(Clone)] перед объявлением типа. Раст сам сгенерирует реализацию
// Clone и склонирует все поля. Кстати нужно чтоб все поля типа были также клонируемы.
// Эти аннотации к типам называются атрибутами.
impl Clone for BigInt {
    fn clone(&self) -> Self {
        BigInt { data: self.data.clone()}
    }
}

// We can also make the type `SomethingOrNothing<T>` implement `Clone`.
// Рассмотрим второй arm. Почему не Something(v)? В таком случае это
// означало бы что мы владеем v в коде после =>, но это не сработает
// мы должны оставить владение тому кто вызвал нас. Мы также не владеем
// self просто позаиствовали его. Дописав слово ref мы заимствуем v 
// на время действия arm. Этого хватит для клонирования.
use part02::{SomethingOrNothing,Something,Nothing};
impl<T: Clone> Clone for SomethingOrNothing<T> {
    fn clone(&self) -> Self {
        match *self {
            Nothing => Nothing,
            Something(ref v) => Something(v.clone()),
        }
    }
}

// **Exercise 05.2**: Write some more functions on `BigInt`. What about a function that returns the
// number of digits? The number of non-zero digits? The smallest/largest digit? Of course, these
// should all take `self` as a shared reference (i.e., in borrowed form).

// ## Mutation + aliasing considered harmful (part 2)
enum Variant {
    Number(i32),
    Text(String),
}

// n будет ссылкой на чать var, а так как написали ref mut то ссылка будет mut ссылкой
// ptr в свою очередь будет ссылкой на число сохраненное в var
// если var это Number.
// 
// Когда нам разрешают мутировать var. Могли бы сделать его текстом. Но ptr по прежнему
// будет указывать на старое местоположение! Следовательно ptr теперь указываеткуда-то
// на представление строки. Измемняя ptr мы манипулируем строкой непредсказуемым образом
// и может случиться все что угодно. Технически вообще первое поле String это указатель на
// символы, поэтому перезаписывая этот указатель целым числом мы делаем его полностью
// недействительным адресом и когдадесттруктор var запустится он попытается освободить этот адрес
// и Раст может сделать что угодно.
fn work_on_variant(mut var: Variant, text: String) {
    let mut ptr: &mut i32;
    match var {
        Variant::Number(ref mut n) => ptr = n,
        Variant::Text(_) => return,
    }
    /* var = Variant::Text(text); */                                /* BAD! */
    *ptr = 1337;
}

