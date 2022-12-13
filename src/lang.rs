// Array of sentences by languages
//
// (T, T, ...) is a Tupla, is like an Array but can be destruct, like that
// let (a, b, c, ...) = tupla;
//
// and works like an Array, for example
// let a = tupla.0;
pub const LANGUAGES: &[(&str, &[&str])] = &[
    (
        "es",
        &[
            "Que margen tendra el arbol? (20 por defecto)",
            "Escribe el tamaño del arbol",
            "Quieres continuar? [y|n]",
            "El tamaño del arbol debe ser mayor a 0",
        ],
    ),
    (
        "en",
        &[
            "What margin will the tree have (20 by default)",
            "Type the size of the tree",
            "Do you want to continue? [y|n]",
            "The size of the tree must be greater than 0",
        ],
    ),
];

// verify if language input exists on list
pub fn is_language_valid(s: &str) -> bool {
    LANGUAGES.iter().any(|l| l.0 == s)
}
