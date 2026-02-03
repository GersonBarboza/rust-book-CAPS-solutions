fn main() {
    let admin: Admin = User::new("Dom Corleone");

    println!("Bem-vindo usuário {}.", admin.username());
    println!("{}", admin.login());
    println!("{}", admin.logout());

let operator: Operator = User::new("Januário Operário");

    println!("Bem-vindo usuário {}.", operator.username());
    println!("{}", operator.login());
    println!("{}", operator.logout());

let basic_user: BasicUser = User::new("João da Silva");

    println!("Bem-vindo usuário {}.", basic_user.username());
    println!("{}", basic_user.login());
    println!("{}", basic_user.logout());


}


trait User {
    //Construtor que recebe o nome do usuário (login)
    fn new(username : &'static str) -> Self;

    //Retona login definido em new
    fn username(&self) -> &'static str;

    //Loga no sistema
    fn login(&self) -> &'static str;

    //Desloga do sistema
    fn logout(&self) -> &'static str;

    //Verifica se está logado
    fn is_logged_in(&self) -> bool {
        false
    }
}

struct Admin {
    username: &'static str
}

struct Operator {
    username: &'static str
}

struct BasicUser {
    username: &'static str
}

impl User for Admin{
    fn new(username :&'static str) -> Admin{
        Admin{ username: username}
    }
    fn username(&self) -> &'static str {
        self.username
    }
    fn login(&self) -> &'static str {
        "Usuário do tipo ADMIN entrou no sistema."
    }
    fn logout(&self) -> &'static str {
        "Usuário do tipo ADMIN saiu do sistema."
    }
}

impl User for Operator{
    fn new(username :&'static str) -> Operator{
        Operator{ username: username}
    }
    fn username(&self) -> &'static str {
        self.username
    }
    fn login(&self) -> &'static str {
        "Usuário do tipo OPERATOR entrou no sistema."
    }
    fn logout(&self) -> &'static str {
        "Usuário do tipo OPERATOR saiu do sistema."
    }
}

impl User for BasicUser{
    fn new(username :&'static str) -> BasicUser{
        BasicUser{ username: username}
    }
    fn username(&self) -> &'static str {
        self.username
    }
    fn login(&self) -> &'static str {
        "Usuário do tipo BÁSICO entrou no sistema."
    }
    fn logout(&self) -> &'static str {
        "Usuário do tipo BÁSICO saiu do sistema."
    }
}




