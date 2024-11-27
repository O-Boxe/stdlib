# fsp

fsp or file system plus is a crates that provides some function that help you to create, upload, delete files

## create file

### `create_file()`

he take 2 parametres :

1. `content` : the content of the file (string)
2. `to_path` : the path of the file to put the content in, you don't need to check if the folder is created because the function already check if  the folders of the output path is already created

return type :

* `Result<(), Box<dyn std::error::Error>>`

```rust
match create_file(
            String::from("hello wordl"),
            "../uploads/testDeCreation/001.txt"
        ) {
            Ok(_) => {
                println!("Fichier crée avec succès !")
            }
            Err(e) => eprintln!("Erreur : {}", e),
        }
```

## upload a file

for uploading a file there is 2 functions :

- upload_file_from
- upload_file_from_cb

### `upload_file_from()`

he take 2 parametres :

1. `from_path` : the path of the file that you want to upload
2. `to_path` : the path of the file to put the content in, you don't need to check if the folder is created because the function already check if  the folders of the output path is already created

return type :

* `Result<(), Box<dyn std::error::Error>>`

exemple :

```rust
match upload_file_from(
            "https://server11.mp3quran.net/hazza/001.mp3",
            "../uploads/test/001.mp3"
        ) {
            Ok(_) => {
                println!("File upload successfully !")
            }
            Err(e) => eprintln!("Error : {}", e),
        }
```

### `upload_file_from_cb()`

he take 3 parametres :

1. `from_path` : the path of the file that you want to upload
2. `to_path` : the path of the file to put the content in, you don't need to check if the folder is created because the function already check if  the folders of the output path is already created
3. `callback` : a callback with 1 parametres, the passed parameters is a f64 and that is the progress of the downloaded file

return type :

* `Result<(), Box<dyn std::error::Error>>`

exemple :

```rust
match upload_file_from_cb(
            "https://server11.mp3quran.net/hazza/001.mp3",
            "../uploads/test/001.mp3",
            |progress|{println!("{}%", progress)}
        ) {
            Ok(_) => {
                println!("File upload successfully !")
            }
            Err(e) => eprintln!("Erreur : {}", e),
        }
```

## delete a file/folder

for deleting there is 2 functions :

- delete_file
- delete_folder
- delete_fof

### `delete_file()`

he take 1 parametres :

1. `target_path` : the path of the file

return type :

* `io::Result<()>`

exemple :

```rust
match delete_file("") {
            Ok(_) => println!("File successfully deleted !"), 
            Err(e) => eprintln!("Error : {}", e) 
        }
        //or
        delete_file("../uploads/testDeCreation/001.txt").expect("Error");
```

### `delete_folder()`

he take 1 parametres :

1. `target_path` : the path of the folder

return type :

* `io::Result<()>`

exemple :

```rust
match delete_folder("") {
            Ok(_) => println!("File successfully deleted !"), 
            Err(e) => eprintln!("Error : {}", e) 
        }
        //or
        delete_folder("../uploads/testDeCreation/001.txt").expect("Error");
```

### `delete_fof()` (fof = file or folder)

he take 1 parametres :

1. `target_path` : the path of the folder or the folder

return type :

* `io::Result<()>`

exemple :

```rust
match delete_fof("") {
            Ok(_) => println!("File successfully deleted !"), 
            Err(e) => eprintln!("Error : {}", e) 
        }
        //or
        delete_fof("../uploads/testDeCreation/001.txt").expect("Error");
```
