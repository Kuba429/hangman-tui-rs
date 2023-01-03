pub const ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

// PLEASE NOTE THAT THE DRAWING LOOKS DIFFERENCT WHEN RENDERED
pub const HANGMAN_STAGES: [&str; 7] = [
    "
  +------+
  |      |
  |      |
  o      |
/|\\     |
/ \\     |
         |
        =======
",
    "
  +------+
  |      |
  |      |
  o      |
/|\\     |
         |
         |
        =======
",
    "
  +------+
  |      |
  |      |
  o      |
         |
         |
         |
        =======
",
    "
  +------+
         |
         |
         |
         |
         |
         |
        =======
",
    "
          
         |
         |
         |
         |
         |
         |
        =======
",
    "
          
          
          
          
          
          
          
        =======
",
    "
          
          
          
          
          
          
          
               
",
];
