
use serde::{Serialize,Deserialize};


#[derive(Deserialize)]
struct Club{
name:String,
other_members:Vec<String>
}

#[derive(Deserialize)]
struct Student{
    name:String,
    usn:String,
    clubs:Vec<Club>
}

fn main(){
    let json=r#"
                    {

                        "name":"sathwik kd",
                        "usn":"4AL21EC029",

                        "clubs":[ {
                            "name":"chirrp",
                            "other_members":["gowtham","sumith ks","deepak r","vrijulal"]
                        },

                        {
                            "name":"chenda",
                            "other_members":["vaadu","akash","kd boy.."]
                        }
                        
                                                
                         ]
                    }
            "#;

    let s=parse_json(json);

    println!("{} {} {}",s.name,s.usn,s.clubs[1].other_members[0]);
}


fn parse_json(raw_json:&str)->Student{
    let s1:Student=serde_json::from_str(raw_json).unwrap();
    return s1;
}