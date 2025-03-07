pub fn example() {
    // ch7_1();
    ch7_2();
}

fn ch7_1() {
    let my_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let new_vec = my_vec.into_iter().skip(3).take(4).collect::<Vec<i32>>();
    println!("{new_vec:?}");
}

fn ch7_2() {
    let mut vector2 = vec![10, 20, 30];
    vector2.iter_mut().for_each(|x| *x += 100);

    println!("{:?}", vector2);
}

#[derive(Debug)]
struct Library {
    library_type: LibraryType,
    books: Vec<String>,
}

#[derive(Debug)]
enum LibraryType {
    City,
    Country,
}

impl Library {
    fn add_book(&mut self, book: &str) {
        self.books.push(book.to_string())
        //&str 을 가져와 문자열로 바꾼 다음 Vec에 추가합니다.
    }
    fn new() -> Self {
        Self {
            library_type: LibraryType::City,

            books: Vec::new(),
        }
    }
}
fn ch7_2_1() {
    let mut my_library = Library::new();
    my_library.add_book("The Doom of the Darksword");


    
}
