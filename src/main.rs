fn main() {
    let variavel: u8 = 16;
    let responsavel_autorizou = true;
    println!("variavel = {}", variavel);

    if variavel > 18 || responsavel_autorizou {
        println!("Pode entrar na balada!");
    } else {
        println!("Nao pode entrar na balada!");
    }

    // condicionais();
    // tabuada();
    ownership();
}

fn condicionais() {
    let idade: u8 = 18;
    let responsavel_autorizou = true;
    let eh_maior = idade >= 18;

    if eh_maior {
        println!("Pode entrar na balada");
    } else if idade > 16 && responsavel_autorizou {
        println!("Pode entrar com assinatura do responsavel");
    } else {
        println!("Nao pode entrar na balada");
    }

    let condicao = if eh_maior { "maior" } else { "menor" };

    println!("Eh {} de idade", condicao);

    let linguagem = "C";
    let proposito = match linguagem {
        "PHP" => "WEB",
        "Kotlin" => "Android",
        "Python" => "Data Science",
        _ => "Desconhecido"
    };

    println!("O proposito de {} eh {}", linguagem, proposito);
}

fn tabuada() {
    println!("Tabuada de 1");
    for i in 1..11 {
        println!("{} x {} = {}", 1, i, 1 * i);
    }
}

fn ownership() {
    let uma_string = String::from("Vinicius");
    println!("Endereco da string {}", &uma_string);
    rouba(&uma_string);

    println!("{}", uma_string);
}

fn rouba(string: &String) {
    println!("Endereco da segunda string {}", &string);
    println!("{}", string);
}