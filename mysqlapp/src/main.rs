extern crate chrono;
extern crate mysql;

use mysql as my;
use chrono::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct Event {
    id: i32,
    name: String,
    about: String,
    date: chrono::NaiveDateTime
}

fn main() {

    // Connection Pool
    let pool = my::Pool::new("mysql://root:root@localhost:8889").unwrap();

    // Insert new Event
    let events = vec![
        Event {
            id: 13,
            name: "TEST RUST 2".to_string(),
            about: "bla bla bla bla bla bla bla bla bla".to_string(),
            date: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11)
        }
    ];

    for mut stmt in pool.prepare(r"INSERT INTO rustpe.event
                                       (id, name, about, date)
                                   VALUES
                                       (:id, :name, :about, :date)").into_iter() {
        for e in events.iter() {
            stmt.execute((e.id,&e.name,&e.about,e.date,)).unwrap();
        }
    }

    // All Events
    let selected_events: Vec<Event> =
    pool.prep_exec("SELECT id, name, about, date from rustpe.event", ())
    .map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, name, about, date) = my::from_row(row);
            Event {
                id: id,
                name: name,
                about: about,
                date: date
            }
        }).collect()
    }).unwrap();

    println!("Events {:?}", selected_events);

    for elemt in selected_events.iter() {
        println!("ID: {:?} Nombre: {:?}", elemt.id, elemt.name);
    }

    let dt = UTC.ymd(2014, 7, 8).and_hms(9, 10, 11);
    println!("Fecha {:?}", dt);

    println!("Hoy {:?}", Local::now());

    //Delete id 3
    pool.prep_exec(r"delete from rustpe.event where id=:id",(3,)).unwrap();

    //Update id 4
    pool.prep_exec(r"update rustpe.event set about=:about where id=:id",("bla.....".to_string(),4,)).unwrap();

}
