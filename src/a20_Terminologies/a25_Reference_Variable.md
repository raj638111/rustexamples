

# Reference variable

1. Holds the address of some other variable
2. Example
   1. `io::stdin().read_line(&mut guess)`
      1. Here, 
         - `&`: Specifies we are passing the address of some other variable
         - `mut`: Represents we intent to change the value pointed by this variable
         - `guess`: Is the name of the reference variable