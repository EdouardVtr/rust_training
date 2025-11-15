use std::io;     // Importe la bibliothèque d'entrée/sortie "io", cette bibliothèque provient quant à elle de la bibliothèque standart, connue sous le nom "std".

fn main() {     // "fn" déclare une nouvelle fonction appelée "main", les "()" indiquent que cette fonction n'accepte aucun paramètre et l'accolade ouvrante "{" marque le début du corps de la fonction.
  
    println!("Guess the number!");

    println!("Please input your guess.")

    let mut guess = String::new();     // Cette ligne crée une variable mutable nommée guess qui contient une nouvelle chaîne de caractères vide, une instance de string.

    // Recueillir la saisie utilisateur
    io::stdin()    // Permet d'appeler la fonction "stdin" du module "io", qui va nous permettre de traiter la saisie utilisateur. 
        .read_line(&mut guess)    // .read_line appelle la méthode read_line sur l'entrée standard afin d'obtenir la saisie utilisateur.
                                  // (&mut guess) est un argument de read_line qui lui indique dans quelle chaîne de caractères il faut stocker la saisie utilisateur
                                  // le "&" indique que cet argument est une référence, ce qui permet de laisser plusieurs morceaux de notre code accéder à une même donnée.
        .expect("Failed to read line");    // Si l'instance de "io::result" a pour valeur la variante Err, l'appel à expect fera planter le programme et affichera le message chosi : ""Failed to read line".

    println!("You guessed: {}", guess);
}

