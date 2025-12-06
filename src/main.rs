//Imported lybraries and modules
mod crypto;
mod file_reading;
use dialoguer::{Input, Select};

//Main function that will run the whole program
fn main() {
    //making the console programn more friendly 
    //adding some structure
    println!("🔐 Crypto Tool");
    println!("---------------");

    //Ask user for an action to be perform
    //Have a vector with the options the user can select
    let actions = vec!["Encrypt a file", "Decrypt a file", "Exit"];

    //using the dialoguer crate make a interactive command line selection
    //create a new selection menu object, for building an interactive list
    let selection = Select::new()
    //message to be shown to the user
    .with_prompt("Choose an action")
    //provide the list of options that the user can choose from
    //the list is the vector object that was declared before
    //when an option is selected it will be assigned to the variable selection
    .items(&actions)
    //set the default highlighted option, 0 is the first item in the vector
    .default(0)
    //Render the menu in the terminal and waits for the user until they make a choice
    .interact()
    //extract the value of the option selected to assign it to the variable
    .unwrap();

    //create an array with the name of key with a lenght of 32, initialized 
    //to unsigned 8-bit integer
    let key = [0u8; 32];

    //Rust similar method of a switch case
    //each selection will perform one operation
    match selection{
        0 =>{
            //First option is for Encrypting a file

            //Ask the user for the path of the file to be encrypted
            let input_path: String = Input::new()
            .with_prompt("Introduce the path of the file to encrypt please")
            //wait for the user input
            .interact_text()
            //extract the result and save it in the variable input_path
            .unwrap();

            //Ask the user for the path to save the new file Encrypted
            //This input works similar to the one above
            let output_path: String = Input::new()
            .with_prompt("Enter the path to allocate the encrypted file please")
            .interact_text()
            .unwrap();
            
            //Reads the file content into the variable plaintext
            let plaintext = file_reading::read_file(&input_path);
            //Ecrypts the file using the function encrypt in crypto module, we need the text
            //and the key we defined early
            let encrypted = crypto::encrypt(&plaintext, &key);
            //Writes the encrypted data to the output file using the function 
            //write_file in the module file_reading
            file_reading::write_file(&output_path, &encrypted);

            //tell the user the file has been already encrypted
            println!("✅ Your file was encrypted succesfully! -> {}", output_path);
        }
        1 =>{
            //Second option is for Decrypt a file

            //Ask the user for the path of the file to be decrypted
            let input_path: String = Input::new()
            .with_prompt("Enter the path of the encrypted file")
            //wait for the user input
            .interact_text()
            //extract the result and save it in the variable input_path
            .unwrap();

            //Ask the user for the path to save the new fill decrypted
            //This input works similar that the one above
            let output_path: String = Input::new()
            .with_prompt("Enter the output path for the decrypted file")
            .default("decrypted.txt".to_string())
            .interact_text()
            .unwrap();

            //Reads the file content into the variable encrypted_data
            let encrypted_data = file_reading::read_file(&input_path);
            //Decrypt the file using the function decrypt in crypto module, we need the text
            //and the key we defined early
            let decrypted = crypto::decrypt(&encrypted_data, &key);
            //Writes the descrypted data to the output file using the function 
            //write_file in the module file_reading
            file_reading::write_file(&output_path, &decrypted);

            //tell the user the file has been decrypted
            println!("Your file was decrypted succesfully! -> {}", output_path);
        }
        _ =>{
            //last option doesn't do anything only display a bye message
            println!("👋 Thanks for using my program!")
        }
    }
}
