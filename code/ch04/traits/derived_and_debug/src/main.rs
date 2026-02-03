fn main() {
    let admin: Admin = User::new("Dom Corleone");

    // Este "{:?} só é possível por causa da diretiva derive (ou seja, '#[derive(Debug)]' ) 
    // usada antes da definição da estrutura Admin.
    println!("usando 'println! => {:?}", admin);

    println!("usando 'dbg!' ==>");
    dbg!(admin);


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

#[derive(Debug)]
struct Admin {
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




