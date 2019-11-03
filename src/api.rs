use crate::impls::drinks;
use std::sync::{Arc, Mutex};
use rocket::State;

pub struct Api {
    drinks: Arc<Mutex<drinks::Drinks>>
}

#[get("/")]
fn index() -> String {
    return format!("Drinks Drunk!");
}

#[get("/")]
fn drinks_get(drinks: State<Arc<Mutex<drinks::Drinks>>>) -> String {
    let my_drinks = &mut *drinks.lock().unwrap();

    return format!("{:#?}", my_drinks.list(false));
}

// Should use body params.
#[post("/<name>")]
fn drink_post(name: String, drinks: State<Arc<Mutex<drinks::Drinks>>>) -> String {
    let my_drinks = &mut *drinks.lock().unwrap();

    my_drinks.add(name);

    return format!("{:#?}", my_drinks.find_by_id(1));
}

#[get("/<id>")]
fn drink_get(id: usize, drinks: State<Arc<Mutex<drinks::Drinks>>>) -> String {
    let my_drinks = &mut *drinks.lock().unwrap();

    return format!("{:#?}", my_drinks.find_by_id(id));
}

// Should use body params.
#[patch("/<id>")]
fn drink_patch(id: usize, drinks: State<Arc<Mutex<drinks::Drinks>>>) -> String {
    let my_drinks = &mut *drinks.lock().unwrap();

    let drink = my_drinks.find_by_id_mut(id);

    match drink {
        Some(d) => {
            d.increment();

            return format!("{:#?}", d);
        },
        None => {
            return format!("Not found.");
        },
    }
}

// Implement soft/hard delete?
#[delete("/<id>")]
fn drink_delete(id: usize, drinks: State<Arc<Mutex<drinks::Drinks>>>) -> String {
    let my_drinks = &mut *drinks.lock().unwrap();

    my_drinks.delete_by_id(id, false);

    return format!("Fertig.");
}

impl Api {
    pub fn new(drinks: Arc<Mutex<drinks::Drinks>>) -> Api {
        return Api {
            drinks: drinks
        };
    }
    pub fn run(&self) {
        rocket::ignite()
            .mount("/", routes![index])
            .mount("/drinks", routes![drinks_get, drink_post, drink_get, drink_patch, drink_delete])
            .manage(self.drinks.clone())
            .launch();
    }
}
