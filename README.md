# 🦀 rust_training

##Création d'un environnement de travail Rust sous Linux sans cargo :

### 1. Création du dossier projet :

    ```bash
    $ mkdir Documents/RustProjects
    $ cd Documents/RustProjects
    $ mkdir hello_world
    $ cd hello_world
    $ nano main.rs


### 2. Création du fichier source :

    ```bash
    $ nano main.rs  // Une page éditable s'ouvre. Ecrivez le code suivant :

      fn main() {
        println!("Hello, world!");
      }

### 3. Compilation du programme :

    ```bash
    $ rustc main.rs  //  Cette commande compile le fichier main.rs et crée un exécutable appelé main.


### 4. Exécution du programme :

    ```bash
    $ ./main
    Hello, world!
