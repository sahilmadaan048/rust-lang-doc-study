use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Green"), 50);

    //accessing value in a hash map
    let team_name = String::from("Blue");

    //the get methid returns an option enum Opotion<&v>
    let score = scores.get(&team_name).copied().unwrap_or(0);
    //copied used to convert Option<&i32> to Option<i32>
    //the unwrap_or(0) to set scoreto 0 if scores does not have an entry for the key passed

    //iterate over the hashmap
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    //hash maps and ownership
    //for types that implement the Copy trait,the values get copied into the hashmap
    //for owned types likeSStringn, the values will bemoved and the hash map will be ome the owner of thode values

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // println!("{field_name} "); //comments it or use .clone() methodwhile inseting

    //inserting the refernces to values, the values wont be moved into the hash map,
    //also those references will be valid only as long as the hash map is valid
    /*
        updating in hash map
            1. overwriting a value
            2. adding a key and value only if a ket isn't present
            3. updating a value based in the old value
    */
    //1. overwriting a value
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); //the old value of 25 for the key "Blue" will get overwrittem

    println!("{scores:?}");

    // 2. adding a key and value only if a key is not already present

    let mut scores = HashMap::new();
    scores.insert(String::from("Yellow"), 10);
    scores.insert(String::from("Blue"), 25); //the old value of 25 for the key "Blue" will get overwrittem

    scores.entry(String::from("Green")).or_insert(30); //the key is not present already, thus it will insert the key and  th corrwsoonding value snd return a mutable reference to that value
    scores.entry(String::from("Yellow")).or_insert(30); //it is already present , thus it will just return a mutable refernce to the already existing value

    //the return type of the entry() method is Entry which check sif akey is already present or not
    //
    //the .or_insert() methid on Entry enum ius defined to retunr a mutable reference to the new valye
    println!("{scores:?}");

    //3.updating a value based oin the old value

    let text = "hello world wondeful world !";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; //dereferencing first since the .or_insert() methid finally returns a mutable reference to the valau 
    } //the mutable rederencez go out of score after for loop
    //so all of these changes are safe and are allowed by the borrowingrules

    println!("{map:?}");
    // The split_whitespace method returns an iterator over subslices, separated by whitespace
}

//like the maps we have in c++ or dictionaries in python, objects in js
/*

 HashMap<k,v>stores a mapping of keys of type K to values of type V using a hashjjing function
 whuchh determines hoe ir places these keys into the memory


these are the least type of collections used out of the ones we have studied, and thus
it dioes nnot get included automatically in the prelude unlike vectors or strings

these are also homogenous is nature like vectors
means all the keys stiored must be of same type,

*/

//hasinngn fucntions see the imahes and follow
// through the link in imp.text
//
//
