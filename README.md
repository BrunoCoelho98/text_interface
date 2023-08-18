# text_interface

This Rust program simulates a simple department management system. The program allows you to add people to various departments and list the people in each department. It utilizes Rust's `HashMap` to organize the data.

## How to Run

1. Ensure you have Rust installed on your system.
2. Clone this repository or create a new Rust project with the provided code.
3. Open a terminal and navigate to the project directory.
4. Compile and run the program using the following command:

   ```shell
   cargo run


Usage
The program offers a simple command-line interface for managing departments and people within them. You can perform the following actions:

Add a Person: To add a person's name to a specific department, use the following command:

add <name> to <department>

Make sure to include the "to" keyword after the name.

List All Departments and People: To list all departments and the people in them, use the following command:

  ```shell
  list
````


List People in a Specific Department: To list the people in a specific department, use the following command:

  ```shell
  list <depart>
````
Quit the Program: To exit the program, use the following command:

  ```shell
  quit
  ````

Note: Error Handling
If you attempt to add a person without specifying a department using the "add" command, the program will display an error message indicating that a department should be specified. Make sure to provide both a name and a department using the correct syntax.

Implementation Details
The program uses Rust's HashMap to store departments as keys and a vector of people as values. Each person is associated with a department. You can add people to departments and list the departments along with their respective people.

The main logic is implemented using a loop that continuously reads user input and processes the corresponding commands. The program also includes error handling to provide feedback in case of incorrect input.
