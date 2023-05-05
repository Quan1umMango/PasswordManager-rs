use clap::Parser;
use std::fs;
use clap::Subcommand;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;
use  copypasta::{ClipboardContext,ClipboardProvider};

#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
   #[command(subcommand)]
    command: Commands,
}


#[derive(Subcommand,Debug)]
enum Commands {

    // Initialize everything
    Init,

    // Create a new password
    Create {
        #[arg(short,long)]
        dir: String,
        // Name/Alias the password is for. You can refer the password through this alias/name.
        #[arg(short,long)]
        name: String,
        // The password
        #[arg(short,long)]
        password:String
    },

    // Outputs the password as well as copies it onto your clipboard
    GetPassword {
        #[arg(short,long)]
        dir: String,
        
        #[arg(short,long)]
        name: String
    },

    // Delete a password with the given alias/name.
    Delete {
        // Main folder of the password
        dir: String,
        //Name/Alias of the password
        name: String
    },

}


fn check_for_init() -> bool {
     Path::new("./passwords").is_dir() 
}

fn init() {
    println!("Initalizing...");        
   fs::create_dir("./passwords").unwrap();
    println!("Initalizing Complete.");
}

fn create(dir:String,name:String,password:String) -> std::io::Result<()> {
        let name = name.trim();
        if !check_for_init() {
        println!("Not Initialized. Running the init command");
          init();
        }
      match OpenOptions::new().append(true).open(format!("./passwords/{}/passwords.txt",dir)){
        Ok(mut file) => {
            println!("Found File.");
            file.write_all(format!("\r\n{} :# {}",name,password).as_bytes())?; 
        },
        _=> {
            println!("File/Directory not found, creating a new one.");
            fs::create_dir(format!("./passwords/{}",dir))?;
            println!("Created directory/file.");
           fs::write(format!("./passwords/{}/passwords.txt",dir),format!("{} :*^$@&#^!* {}",name, password))?;
            println!("Sucessfully wrote password for name: {name}");
        }
    }
          
    println!("Great Success!");
    Ok(())
}

fn return_password(dir:String,name:String) -> std::io::Result<String> {
    let file = fs::read_to_string(format!("./passwords/{}/passwords.txt",dir))?;

    for line in file.trim().lines() {
        let line: Vec<&str> = line.split(' ').collect();
        if line[0].to_owned() == name {
            return Ok(line[2].to_owned());
        }
    }
    return Ok("No Password for the given name.".to_owned())
    
}
 
fn get_password(dir: String, name:String) {
   match return_password(dir,name) {
        Ok(password) =>{ 
            let mut ctx  = ClipboardContext::new().unwrap();
            ctx.set_contents(password.to_owned()).unwrap();
            println!("Succesfully copied to clipboard");
        },
        Err(err) => panic!("{err}")
    } 
}


fn delete(dir:String,name:String) -> std::io::Result<()> {
    let file = fs::read_to_string(format!("./passwords/{}/passwords.txt",dir))?;
    let mut fin =  "".to_owned();
    for line in file.lines() {
        let line: Vec<&str> = line.split(' ').collect();
        if line[0].to_owned() != name {
            let mut finline = "".to_owned();
            for l in line {
                finline = format!("{}{}",finline,l);
            }
            fin = format!("{}\r\n{:?}",fin,finline);
        } 
    }
    fs::write(format!("./passwords/{}/passwords.txt",dir),fin)?;
    println!("Sucessfully Deleted.");
    Ok(())
}

fn main() {
   let args = Cli::parse();
   match args.command {
        Commands::Init => {
           if !check_for_init() {
                init()
            }else {
                println!("Already Initialized.")
            }
        },
        Commands::Create{dir,name,password} => {
            let out =  create(dir,name,password); 
           match out  {
                Err(er) => println!("Something went wrong trying to create files: {}",er),
                _=> ()
            } 
        },
        Commands::GetPassword{dir,name} => {
            get_password(dir,name);
        },
        Commands::Delete{dir,name} => {
            let out = delete(dir,name); 
            match out {
            Err(er) => println!("Something went wrong trying to delete: {}",er),
            _=>()
        }
        }
           }; 
}
