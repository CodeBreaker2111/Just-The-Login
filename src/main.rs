use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Hello {
    age: String,
    name: String,
    get_id: bool,
    id: String,
    pasword: String,
    admin_pasword: String
}

fn main()
{
    let mut balls = String::new();
    let mut var = String::new();
    let boll = false;

    let data = std::fs::read_to_string("hello.json").unwrap();

    let mut temp: Hello = serde_json::from_str(&data).unwrap();

    println!("Hello {}\nEnter Pasword: ", temp.name);
    let input = std::io::stdin();
    let mut get_pass = String::new();
    input.read_line(&mut get_pass).unwrap();

    println!();

    if get_pass.trim() == temp.pasword
    {
        if temp.get_id == true
        {
            get_pass.clear();

            println!("Enter 2-step verification ID.");
            input.read_line(&mut get_pass).unwrap();

            if get_pass.trim() == temp.id
            {
                println!("Correct ID.");
            }
            else
            {
                println!("Incorrect ID.");
                return;
            }
        }

        println!("Correct pasword.");
    }
    else
    {
        println!("Incorrect pasword.");
        return;
    }
    println!("Type 1 to add acount.\nType 2 to change pasword.\nType 3 to turn on/change 2-step verification.\nType 4 to turn off 2-step verification.\nType 5 to PERMENTLY clear acount.");
    input.read_line(&mut var).unwrap();

    if var.trim() == "5"
    {
        get_pass.clear();
        var.clear();

        println!("Enter admin pasword to permently clear acount.");
        input.read_line(&mut get_pass).unwrap();
        var = get_pass.trim().to_string();

        if var == temp.admin_pasword
        {
            get_pass.clear();
            var.clear();

            println!("Correct Admin Pasword!");
            println!("Enter 1 to PERMENTLY clear acount.(this will reset paswords and names to 0 and turn of 2-step verification.)");
            input.read_line(&mut get_pass).unwrap();
            var = get_pass.trim().to_string();

            if var == "1"
            {
                println!("Acount Cleared.");

                temp.age = "0".to_string();
                temp.name = "0".to_string();
                temp.get_id = false;
                temp.id = "0".to_string();
                temp.pasword = "0".to_string();
                temp.admin_pasword = "0".to_string();
            }
            else
            {
                println!("Acount unchanged.")
            }
        }
    }

    if var.trim() == "4"
    {
        if temp.get_id == true
        {
            get_pass.clear();
            var.clear();

            println!("Enter admin pasword to turn off 2-step verification.");
            input.read_line(&mut get_pass).unwrap();
            var = get_pass.trim().to_string();

            if var == temp.admin_pasword
            {
                get_pass.clear();
                var.clear();

                println!("Correct Admin Pasword!");
                println!("Enter 1 To verify you are turning off 2-step verification.");
                input.read_line(&mut get_pass).unwrap();
                var = get_pass.trim().to_string();

                if var == "1"
                {
                    println!("2-step verification inactive.");
                    temp.get_id = false
                }
                else
                {
                    println!("2-step verification still active.");
                }
            }
            else
            {
                println!("Incorrect Admin pasword.");
                return;
            }
        }
        else
        {
            println!("2-step verification already inactive.");
            return;
        }
    }

    var.clear();

    if var.trim() == "3"
    {
        get_pass.clear();
        var.clear();

        println!("Enter admin pasword to turn on 2-step verification.");
        input.read_line(&mut get_pass).unwrap();
        var = get_pass.trim().to_string();

        if var == temp.admin_pasword
        {
            get_pass.clear();

            println!("Enter new ID.");
            input.read_line(&mut get_pass).unwrap();
            temp.id = get_pass.trim().to_string();
            temp.get_id = true;
        }
        else
        {
            println!("Incorrect admin pasword.");
            return;
        }
    }

    if var.trim() == "2"
    {
        get_pass.clear();
        var.clear();

        println!("Enter admin pasword to change pasword.");
        input.read_line(&mut get_pass).unwrap();
        var = get_pass.trim().to_string();

        if var == temp.admin_pasword
        {
            get_pass.clear();

            println!("Enter new pasword.");
            input.read_line(&mut get_pass).unwrap();
            temp.pasword = get_pass.trim().to_string();
        }
        else
        {
            println!("Incorrect admin pasword.");
            return;
        }
    }

    if var.trim() == "1"
    {
        get_pass.clear();

        println!("What is your name?");
        input.read_line(&mut get_pass).unwrap();
        temp.name = get_pass.trim().to_string();

        get_pass.clear();

        println!("Enter new pasword.");
        input.read_line(&mut get_pass).unwrap();
        temp.pasword = get_pass.trim().to_string();

        get_pass.clear();

        println!("Enter new admin pasword.");
        input.read_line(&mut get_pass).unwrap();
        temp.admin_pasword = get_pass.trim().to_string();

        println!("How old are you?");
        input.read_line(&mut balls).unwrap();
        temp.age = balls.trim().to_string();
    }
    std::fs::write("hello.json", &serde_json::to_string_pretty(&temp).unwrap()).unwrap();
}
