# Password Manager-rs
A small, CLI for managing passwords with files as storage system...

## Usage

### Getting Help
```sh 
pass --help
```
### Init
Creates a "passwords" folder, where all passwords are stored.
```sh
pass init
```

### Create 
Creates a new password for specifed folder/category with a name/alias.
```sh
pass create --dir <CATEGORY-NAME> --name <NAME> --password <PASSWORD>
```

### Get Password
Copies the password specifed by folder/category with the given name/alias.
```sh 
pass get-password --dir <CATEGORY-NAME> --name <NAME>
```

### Delete Password
Deletes password.
```sh 
pass delete <CATEGORY-NAME> <NAME>
```

