
use leptos::*;
//use leptos_router::*;
//use serde::{Serialize,Deserialize};


use crate::modules::people::people_comms::*;


// when not logged in have a chatbox where people can chat.
// they enter a username and then they can talk.


// when logged in they can c peoples contact information/fields provided they are on a shared
// project or organization


// give people acces to some data tagged as public ?
// this requires a column with row names.




#[component]
pub fn PeopleList(async_data_peopleb: Resource<(), Vec<Person>>) -> impl IntoView {
    view! {
        <div>
            {move || async_data_peopleb.get().map(|people| {
                people.iter().map(|person| {
                    view! {
                        <div class="HUUMAN">
                            {person.name.clone().unwrap_or_else(|| "Unknown".to_string())}
                        </div>
                    }
                }).collect_view()
            }).unwrap_or_default()}
        </div>
    }
}




