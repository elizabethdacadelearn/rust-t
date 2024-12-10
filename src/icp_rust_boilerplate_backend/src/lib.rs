#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]

struct User{
    name:String,
    id:u64,
    email:String,
    createdat:u64
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Activity{
    id:u64,
    nameofactivity:String,
    description:String,
    created_at:u64,
    

}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct GeneralActivities{
    id:u64,
    nameofactivity:String,
    description:String,
    created_at:u64,
    

}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct ActivityProgress{
    id:u64,
    activityid:u64,
    userid:u64,
    title:String,
    activityprogress:String,
    updated_at:u64,
    

}

impl Storable for User {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for User {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Activity {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Activity {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for ActivityProgress {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for ActivityProgress {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}


impl Storable for GeneralActivities {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for GeneralActivities {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}


//thread
thread_local! {
    static MEMEORY_MANAGER:RefCell<MemoryManager<DefaultMemoryImpl>>=RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );
    static ID_COUNTER:RefCell<IdCell>=RefCell::new(
        IdCell::init(MEMEORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),0).expect("Cannot create a counter")
    );
    static USERS_STORAGE:RefCell<StableBTreeMap<u64,User,Memory>>=RefCell::new(StableBTreeMap::init(
        MEMEORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
    static ACTIVITIES_STORAGE:RefCell<StableBTreeMap<u64,Activity,Memory>>=RefCell::new(StableBTreeMap::init(
        MEMEORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));
    static ACTIVITIESPROGRESS_STORAGE:RefCell<StableBTreeMap<u64,ActivityProgress,Memory>>=RefCell::new(StableBTreeMap::init(
        MEMEORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));
    static GENERALACTIVITIES_STORAGE:RefCell<StableBTreeMap<u64,GeneralActivities,Memory>>=RefCell::new(StableBTreeMap::init(
        MEMEORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));
}

#[derive(candid::CandidType,Clone,Serialize,Deserialize,Default)]

struct UserPayload{
    name:String,
    email:String,
}

#[derive(candid::CandidType,Serialize,Deserialize,Default)]
struct ActivityPayload{
    by:u64,
    nameofactivity:String,
    description:String,

}


#[derive(candid::CandidType,Serialize,Deserialize,Default)]

struct SearchPayload{
    activityid:u64,
}


#[derive(candid::CandidType,Serialize,Deserialize,Default)]
struct DeletePayload{
    userid:u64,
    activityid:u64
}


#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct ActivityProgressPayload{
    
    activityid:u64,
    userid:u64,
    title:String,
    activityprogress:String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct UserActivityPayload{
    
   username:String
}

#[derive(candid::CandidType,Deserialize,Serialize)]
enum Errors{
    UserAlreadyFound{msg:String},
    NotFound{msg:String},
    OnyOwner{msg:String},
    MissingCredentials{msg:String}
}


#[ic_cdk::update]
fn register_user(payload: UserPayload) -> Result<User, String> {
    // Validate the payload to ensure that the required fields are present
    if payload.name.is_empty()
        ||payload.email.is_empty()
    {
        return Err("All fields are required".to_string());
    }

    // Validate the payload to ensure that the email format is correct
    if !payload.email.contains('@') {
        return Err("enter correct email format".to_string());
    }

    // Ensure email address uniqueness and ownername and also transport name
    let email_exists:bool = USERS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_, val)| val.email == payload.email)
    });
    if email_exists {
        return Err("Email already exists".to_string());
    }

   let name_exists:bool=USERS_STORAGE.with(|storage| {
    storage
        .borrow()
        .iter()
        .any(|(_,val)| val.name == payload.name)
});
if name_exists {
    return Err("The name already exists".to_string());
}
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let newuser = User {
        id,
        name: payload.name,
        email:payload.email,                                    
        createdat: time(),
       
    };

    USERS_STORAGE.with(|storage| storage.borrow_mut().insert(id, newuser.clone()));

    Ok(newuser)
}

//add an activity

#[ic_cdk::update]
fn add_an_activity(payload:ActivityPayload)->Result<Activity,String>{

      // Validate the payload to ensure that the required fields are present
      if  payload.nameofactivity.is_empty()
      || payload.description.is_empty()
       {
          return Err("All fields are required".to_string());
       }
    

    //check if user is registered
    let user =USERS_STORAGE.with(|storage| storage.borrow().get(&payload.by));
    match user {
        Some(_) => (),
        None => return Err("you are not registered.".to_string()),
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");
    let new_activity=Activity{
        id,
        nameofactivity:payload.nameofactivity,
        description:payload.description,
        created_at:time()    };

   ACTIVITIES_STORAGE.with(|storage| storage.borrow_mut().insert(id, new_activity.clone()));

    Ok(new_activity)
}

//retrive all activities

#[ic_cdk::query]
fn get_all_user_activities(payload:UserActivityPayload) -> Result<Vec<Activity>, String> {

      //verify username exists

      if payload.username.is_empty(){
        return Err("username is required.".to_string())
      }

    let activities =ACTIVITIES_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .map(|(_, trans)| trans.clone())
            .collect::<Vec<Activity>>()
    });

    if  activities.is_empty() {
        return Err("No activities   found.".to_string());
    }

    else {
        Ok(activities)
    }
}

//get an activity
#[ic_cdk::query]
fn get_an_activity_detail(payload:SearchPayload)->Result<Activity,String>{
    let activity =ACTIVITIES_STORAGE.with(|storage| storage.borrow().get(&payload.activityid));
    match activity {
        Some(act) => Ok(act),
        None => Err("something went wrong no activity found.".to_string()),
    }
}


//remove an activity

#[ic_cdk::update]
  fn remove_an_activity(payload:DeletePayload)->Result<String,String>{
 //verify  its the owner
   let owner =USERS_STORAGE.with(|storage| storage.borrow().get(&payload.userid));
    match owner {
        Some(_) => (),
        None => return Err("must be the owert.".to_string()),
    }
    match ACTIVITIES_STORAGE.with(|storage|storage.borrow_mut().remove(&payload.activityid)){
        Some(_val)=>Ok("tou have eleted an ctivity".to_string()),
        None=>Err("coulde not delete".to_string(),)
    }
  }


  //update your activiteis to show your progress

  #[ic_cdk::update]
fn users_update_activities_progress(payload:ActivityProgressPayload)->Result<ActivityProgress,String>{
     if payload.title.is_empty()
        ||payload.activityprogress.is_empty()
    {
        return Err("Ensure all credentials are inserted".to_string());
    }
    

    //validate if user is registerde
    let user_exists:bool = USERS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_, val)| val.id == payload.userid)
    });
    if !user_exists {
        return Err("must be registered".to_string());
    }
    //validate if activiy exists

    let activity_exists:bool = ACTIVITIES_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_, val)| val.id == payload.activityid)
    });
    if !activity_exists {
        return Err("activity with given id".to_string());
    }

//create a new progress
let id = ID_COUNTER
.with(|counter| {
    let current_value = *counter.borrow().get();
    counter.borrow_mut().set(current_value + 1)
})
.expect("Cannot increment ID counter");

let newprogress =ActivityProgress {
id,
activityid:payload.activityid,
userid:payload.userid,
title:payload.title,
activityprogress:payload.activityprogress,
updated_at:time() 

};
ACTIVITIESPROGRESS_STORAGE.with(|storage| storage.borrow_mut().insert(id, newprogress.clone()));

Ok(newprogress)

}


//general activities


#[ic_cdk::update]
fn add_general_activity(payload:ActivityPayload)->Result<GeneralActivities,String>{

      // Validate the payload to ensure that the required fields are present
      if  payload.nameofactivity.is_empty()
      || payload.description.is_empty()
       {
          return Err("All fields are required".to_string());
       }
    

    //check if user is registered
    let user =USERS_STORAGE.with(|storage| storage.borrow().get(&payload.by));
    match user {
        Some(_) => (),
        None => return Err("you are not registered.".to_string()),
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");
    let new_activity=GeneralActivities{
        id,
        nameofactivity:payload.nameofactivity,
        description:payload.description,
        created_at:time()    };

   GENERALACTIVITIES_STORAGE.with(|storage| storage.borrow_mut().insert(id, new_activity.clone()));

    Ok(new_activity)
}

//retrive all activities

#[ic_cdk::query]
fn get_all_general_activities() -> Result<Vec<GeneralActivities>, String> {

     

    let activities =GENERALACTIVITIES_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .map(|(_, trans)| trans.clone())
            .collect::<Vec<GeneralActivities>>()
    });

    if  activities.is_empty() {
        return Err("No activities   found.".to_string());
    }

    else {
        Ok(activities)
    }
}


ic_cdk::export_candid!();
