use reqwest::blocking::Client;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;


/* -----------------------------------------------create file----------------------------------------------- */

pub fn create_file(content: String, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Ensure the folder exists
    ensure_folder_exists(output_path)?;

    // Create the file and open it for writing
    let mut file = File::create(output_path)?;

    // Write the content to the file
    file.write_all(content.as_bytes())?;

    Ok(())
}

/* -----------------------------------------------update file----------------------------------------------- */
pub fn upload_file_from_cb<F>(
    from_path: &str,
    to_path: &str,
    file_name: &str,
    callback: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn(f64),
{
    // get file
    let client = Client::new();
    let mut response = client.get(from_path).send()?;

    let total_size = response
        .content_length()
        .ok_or("Failed to get content length")?;

    
    // ensure folder exist
    ensure_folder_exists(to_path)?;

    // create file
    let file_path = format!("{}{}", to_path, file_name);
    let mut file = File::create(file_path)?;

    // upload avec progression
    let mut downloaded: u64 = 0;
    let mut buffer = [0; 8192];
    loop {
        let bytes_read = response.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // Download complete
        }
        
        file.write_all(&buffer[..bytes_read])?;
        downloaded += bytes_read as u64;

        // Calculate progress and call the callback
        let progress = (downloaded as f64 / total_size as f64) * 100.0;
        callback(progress);
    }


    Ok(())
}

pub fn upload_file_from(
    from_path: &str,
    to_path: &str,
    file_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    upload_file_from_cb(from_path, to_path, file_name, |_| {})
}


/* -----------------------------------------------delete file----------------------------------------------- */
pub fn delete_file(target_path: impl AsRef<str>) -> io::Result<()> {
    // Convert the target path to a &str
    let path = target_path.as_ref();

    std::fs::remove_file(path)?;

    Ok(())
}
pub fn delete_folder(target_path: impl AsRef<str>) -> io::Result<()> {
    // Convert the target path to a &str
    let path = target_path.as_ref();

    std::fs::remove_dir_all(path)?;

    Ok(())
}

pub fn delete_fof(target_path: impl AsRef<str>) -> io::Result<()> {
    let path = Path::new(target_path.as_ref());

    if path.is_dir() {
        // Supprime le dossier et son contenu
        std::fs::remove_dir_all(path)?;
    } else if path.is_file() {
        // Supprime le fichier
        std::fs::remove_file(path)?;
    } else {
        // Le chemin n'est ni un fichier, ni un dossier
        return Err(io::Error::new(io::ErrorKind::NotFound, "Fichier ou dossier introuvable").into());
    }

    Ok(())
}


/* ------------------------------------------------utils------------------------------------------------ */
fn ensure_folder_exists(folder_path: impl AsRef<str>) -> io::Result<()> {
    let path = folder_path.as_ref();

    // Créez un chemin Path à partir de la chaîne de caractères
    let path = Path::new(path);

    // Si le chemin est un fichier, obtenir son dossier parent
    if let Some(parent) = path.parent() {
        // Crée le dossier parent si nécessaire
        std::fs::create_dir_all(parent)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        match upload_file_from(
            "https://server11.mp3quran.net/hazza/001.mp3",
            "../uploads/audio",
            "001.mp3"
        ) {
            Ok(_) => {
                println!("Fichier téléchargé avec succès !")
            }
            Err(e) => eprintln!("Erreur : {}", e),
        }

        match upload_file_from_cb(
            "https://server11.mp3quran.net/hazza/013.mp3",
            "../uploads/audio",
            "013.mp3",
            |progress| {println!("progression : {}%", progress)}
        ) {
            Ok(_) => {
                println!("Fichier téléchargé avec succès !")
            }
            Err(e) => eprintln!("Erreur : {}", e),
        }


        match create_file(
            String::from("hello wordl"),
            "../uploads/testDeCreation/001.txt"
        ) {
            Ok(_) => {
                println!("Fichier crée avec succès !")
            }
            Err(e) => eprintln!("Erreur : {}", e),
        }

        match delete_fof("../uploads/testDeCreation/001.txt") {
            Ok(_) => println!("File successfully deleted !"), 
            Err(e) => eprintln!("Error : {}", e) 
        }

    }
}
