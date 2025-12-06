//crates to be used
use aes_gcm::{Aes256Gcm, Nonce};                 // Concrete cipher + nonce
use aes_gcm::aead::{Aead, KeyInit};              // Traits for new/encrypt/decrypt
use rand::Rng;

//function to encrypt a file, the function will return a vector
pub fn encrypt(data: &[u8], key: &[u8; 32]) -> Vec<u8> {
    // Converts the raw [u8; 32] key into a proper type to be used for AES-256-GCM
    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(key);
    // Creates a cipher instance bound to the key
    // This object will be used to encrypt and decrypt
    let cipher = Aes256Gcm::new(key);

    // Number used once for encrypting
    // An array of 12 elements
    // AES-256-GCM requires a Nonce of 12 bytes
    //We initialize all the elements with 0
    let mut nonce_bytes = [0u8; 12];
    //We rewrite the 12 initial 0s of the variable above with 12 bytes of data
    //generated randomly, this is the generated nonce
    rand::thread_rng().fill(&mut nonce_bytes);
    //this variable will be the one be used to create encrypting operation
    //we will use it with the object cipher
    let nonce = Nonce::from_slice(&nonce_bytes);

    //We make the encription and saved it in the variable ciphertext
    //We send the data and nonce to the method encrypt to perform the operation
    //The variable will save the encrypted text and the authentication tag
    let ciphertext = cipher.encrypt(nonce, data).expect("Encryption failed");


    //We declare a new variable called out the nonce_bytes array will be change
    //to a vector; the variable will contain only the 12 bytes of the nonce
    let mut out = nonce_bytes.to_vec();
    //we add the authentication tag and the encrypted text to the out variable
    //to the end of the vector, now the vector will have the nonce, the encrypted text
    //and the tag
    out.extend(ciphertext);
    //return the out variable
    out
}

//function to decrypt files, the function will return a vector
pub fn decrypt(encrypted_data: &[u8], key: &[u8; 32]) -> Vec<u8> {
    // We split the data of the object ecrypted_data in two parts, the first 12 bytes are
    // assigned to the nonce_bytes variable, and the rest to the ciphertext variable
    let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
    // Convert the nonce_bytes to the type nonce that the algorithm requires
    let nonce = Nonce::from_slice(nonce_bytes);

    // Converts the raw [u8; 32] key into a proper type to be used for AES-256-GCM
    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(key);
    // Create the encrypting object using the key generated
    let cipher = Aes256Gcm::new(key);

    // Call the method decrypt passing to it the parameters nonce and ciphertext that are required
    // If an error occurs the function will panic and the error will be displayed
    cipher.decrypt(nonce, ciphertext).expect("Decryption failed")
}