pub mod logic {
    use std::collections::HashMap;

    fn obtener_valores_curp() -> HashMap<char, u32> {
        let caracteres = "0123456789ABCDEFGHIJKLMNÑOPQRSTUVWXYZ";
        let mut mapa = HashMap::new();
        for (i, c) in caracteres.chars().enumerate() {
            mapa.insert(c, i as u32);
        }
        mapa
    }

    fn calcular_digito_verificador(curp: &str) -> Option<u32> {
        if curp.len() != 18 {
            return None;
        }

        let mapa = obtener_valores_curp();
        let curp = curp.to_uppercase();
        let chars: Vec<char> = curp.chars().collect();

        let mut suma: u32 = 0;
        for i in 0..17 {
            let c = chars[i];
            let valor = *mapa.get(&c)?;
            let peso = 18 - i as u32;
            suma += valor * peso;
        }

        let residuo = suma % 10;
        let digito = if residuo == 0 { 0 } else { 10 - residuo };

        Some(digito)
    }

    pub fn verificar_curp(curp: &str) -> bool {
        if curp.len() != 18 {
            return false;
        }

        match calcular_digito_verificador(curp) {
            Some(digito_calculado) => {
                let digito_en_curp = curp.chars().nth(17).unwrap();
                match digito_en_curp.to_digit(10) {
                    Some(d) => d == digito_calculado,
                    None => false,
                }
            }
            None => false,
        }
    }
}