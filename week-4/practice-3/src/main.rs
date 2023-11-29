n main() {
    let name1 = "Abbah Emmanuel";
    println!("My name is {}",name1);

    //find and replace
    let name2 = name1.replace("Abbah", "Owocho");
    println!("You can also call me {}",name2);
    let faculty = "Faculty of Science and Teachnology";

    //find and replace
    let school = faculty.replace("Faculty", "School");
    println!("I am a student of the {}", school);

}