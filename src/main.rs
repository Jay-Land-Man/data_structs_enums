fn main() {
    let mut user1 = User {
        email: String::from("someone@example.net"),
        username: String::from("manbearpig"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("thisisactually@myemail.com");
    // notice how when you make this kind of object you 
    // can alter it in this manner

    let _user2 = build_user(String::from("danemail.com"), 
                           String::from("dantheman"));

    let user4 = User {
        email: String::from("janks@bruski"),
        username: String::from("wallacethejew"),
        ..user1
    };

    println!("User: {} \n email: {} \n active: {} \n sign in count: {}", 
            user4.username, user4.email, user4.active, user4.sign_in_count);

    // going to now use a different strut
    let black = Colour(0, 0, 0);

    println!("blacks colour pixels are: {:?}", black);

   // the given example program for structs
   // a rectangle is a single object
   // despite having two properties that describe it
    // if we make it into a struct and 
    // then we can also take advantage of
    // explicit referencing
    let rect = Rectangle {
        height: 30,
        width: 50,
    };
    let rect2 = Rectangle {
        height: 50,
        width: 80,
    };

    // we can go one better on the whole area
    // thing and add a method to the struct
    // class called area
    println!("the area of the rectangle is: {}", rect.area());
    println!("additionally the rectagle is: {:?}", rect);
    println!("can rectangle two hold one?: {}", rect2.can_hold(&rect));
    // as we can see this is much much neater
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width *self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Colour(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32,
}