use stylus::{account, contract, deploy};

#[contract]
pub struct RSV {
    next_token_id: u64,
    owner: account::Address,
}

impl RSV {
    // Constructor del contrato
    pub fn new(owner: account::Address) -> Self {
        RSV {
            next_token_id: 0,
            owner,
        }
    }

    // Función para obtener la URI base
    pub fn base_uri() -> &'static str {
        "https://gateway.lighthouse.storage/ipfs/bafkreibzidfnwztjteak3wip3lodfrd366omc4nhmmwypw4chaorwii5aa"
    }

    // Función para realizar el minting de un token
    pub fn safe_mint(&mut self, to: account::Address) {
        if self.owner != to {
            panic!("Only the owner can mint tokens");
        }

        let token_id = self.next_token_id;
        self.next_token_id += 1;

        // Aquí se simula el minting, en la práctica debes tener la lógica para asociar el token con la dirección
        println!("Minting token with ID {} to address {:?}", token_id, to);
    }
}

// Función para desplegar el contrato en la blockchain
#[deploy]
pub fn deploy(owner: account::Address) -> RSV {
    RSV::new(owner)
}
