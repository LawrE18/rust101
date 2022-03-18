// Rust-101, Part 04: Ownership, Borrowing, References
// ===================================================

/* проблема данного кода в том, что указатель указывает на вектор
 * а когда добавляем элемент с помощью push_back может случиться
 * переаллоцирование вектора, например в случае заполнении старого буфера.
 * Если это случается, то указатель first будет уже недействительным
 * и доступ по нему может сломать программу.
  void foo(std::vector<int> v) {
      int *first = &v[0];
      v.push_back(42);
      *first = 1337; // This is bad!
  }
*/

/* Раст не позволяет такое делать с помощью механизма владения.
 * В примере ниже Раст при передаче вектора в функцию work_on_vector 
 * Раст также с вектором передает владение на вектор.
 * Далее после выхода из функции происходит дроп вектора
 * так как владения над этим векторов находятся уже только
 * в этой функции и значит больше вектор никому не нужен и
 * можно его полностью удалить.
*/
// ## Ownership
fn work_on_vector(v: Vec<i32>) { /* do something */ }
fn ownership_demo() {
    let v = vec![1,2,3,4];
    work_on_vector(v);
    /* println!("The first element is: {}", v[0]); */               /* BAD! */
}

// ## Borrowing a shared reference
/* В предыдущих примерах использовалась функция vec_min. Что будет если вызвать
 * её дважды? После первого раза эта фанкция полностю удалит вектор полностью и второй
 * раз невозможно будет её вызвать и об это сообщит компилятор. 
 * Как можно избежать этого? Ссылки
 * ссылки можно рассматривать как заимствование владения. Есть два вида ссылок.
 * 
 * shared ссылка --- можно дать ссылку любому количеству разных владельцев и все 
 * они могут получить доступ к данным по ней. Это вводит aliasing. поэтому
 * чтобы оставаться также все ёщё безопасным Раст в общем не допускает изменения через shared ссылку.
*/
fn vec_min(v: &Vec<i32>) -> Option<i32> {
    use std::cmp;

    let mut min = None;
    // явно запрашиваем итератор для вектора. Метод предлагает shared ссфлки на элементы вектора. 
    // This time, we explicitly request an iterator for the vector `v`. The method `iter` just
    // borrows the vector it works on, and provides shared references to the elements.
    for e in v.iter() {
        // In the loop, `e` now has type `&i32`, so we have to dereference it to obtain an `i32`.
        // Разымиенование с помощью *.
        min = Some(match min {
            None => *e,
            Some(n) => cmp::min(n, *e)
        });
    }
    min
}

// Now that `vec_min` does not acquire ownership of the vector anymore, 
// we can call it multiple times on the same vector and also do things like
// Создается три shared ссылки (2, 3 и 4 строки функции). Технически ссылки являются
// указателями. Так как vec_min получает только shared ссфлку то Раст знает, что
// эта функция не может изменить вектор. Значит указатель first на буфер вектора 
// останется действительным.
fn shared_ref_demo() {
    let v = vec![5,4,3,2,1];
    let first = &v[0];
    vec_min(&v);
    vec_min(&v);
    println!("The first element is: {}", *first);
}

// ## Unique, mutable references
// Второй тип ссылок --- mut ссылки (изменяемая ссылка). Вместе с ней идет обещание
// что никто другой не имеет доступа к ссылке. Таким образом всегда безопасно изменять
// обхект с помощью этой ссылки. (&mut) 
// В примере ниже нужно использовать iter_mut который предоставляет mut ссылки на элементы,
// чтобы могли их изменять.
fn vec_inc(v: &mut Vec<i32>) {
    for e in v.iter_mut() {
        *e += 1;
    }
}
// Here's an example of calling `vec_inc`.
// Соответственно чтобы пользоваться mut ссылками должны объявить вектор v как mut.
// Не можем создать shared ссфлку на вектор, это происходит потому что функция vec_inc
// может структурно изменить вектор и shared ссылка окажется недействительной.
fn mutable_ref_demo() {
    let mut v = vec![5,4,3,2,1];
    /* let first = &v[0]; */
    vec_inc(&mut v);
    vec_inc(&mut v);
    /* println!("The first element is: {}", *first); */             /* BAD! */
}

// ## Summary
// The ownership and borrowing system of Rust enforces the following three rules:
// 
// * There is always exactly one owner of a piece of data
// * If there is an active mutable reference, then nobody else can have active access to the data
// * If there is an active shared reference, then every other active access to the data is also a
//   shared reference
// 
// As it turns out, combined with the abstraction facilities of Rust, this is a very powerful
// mechanism to tackle many problems beyond basic memory safety. You will see some examples for
// this soon.

